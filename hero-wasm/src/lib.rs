//! Smart Dawn — "Neural Constellation" hero engine.
//!
//! A dependency-free `no_std` particle field compiled to WebAssembly. Every
//! frame the JS shim calls `step(dt)` and then reads the interleaved position
//! buffer (`[x0, y0, x1, y1, ...]`) straight out of linear memory to draw the
//! constellation on a 2D canvas. All physics — drift, edge bounce, pointer
//! gravity well, speed clamping — runs here in Rust. No `wasm-bindgen`, no
//! `libm`: pure arithmetic, a hand-rolled `sqrt`, and a tiny xorshift RNG.

#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

const MAX: usize = 320;
const MAX_LINKS: usize = MAX * (MAX - 1) / 2;

static mut LINKS: [f32; MAX_LINKS * 5] = [0.0; MAX_LINKS * 5];

// ── Simulation state (single-threaded wasm → `static mut` is safe here) ──
static mut POS: [f32; MAX * 2] = [0.0; MAX * 2];
static mut VEL: [f32; MAX * 2] = [0.0; MAX * 2];
static mut N: usize = 0;
static mut W: f32 = 0.0;
static mut H: f32 = 0.0;
static mut PX: f32 = 0.0;
static mut PY: f32 = 0.0;
static mut PACTIVE: bool = false;
static mut RNG: u32 = 0x9E37_79B9;

// ── Small math helpers (core has no f32::sqrt / abs / min in no_std) ──
#[inline]
fn absf(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

#[inline]
fn sqrtf(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    // Fast bit-hack seed, then two Newton–Raphson refinements.
    let i = 0x1fbd_1df5u32.wrapping_add(x.to_bits() >> 1);
    let mut y = f32::from_bits(i);
    y = 0.5 * (y + x / y);
    y = 0.5 * (y + x / y);
    y
}

#[inline]
fn rng_f32() -> f32 {
    // xorshift32 → [0, 1)
    unsafe {
        let mut s = RNG;
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        RNG = s;
        (s >> 8) as f32 / 16_777_216.0
    }
}

#[no_mangle]
pub extern "C" fn seed(s: u32) {
    unsafe {
        RNG = if s == 0 { 0x9E37_79B9 } else { s };
    }
}

/// Initialise `count` particles spread across a `w` × `h` field.
#[no_mangle]
pub extern "C" fn init(count: u32, w: f32, h: f32) {
    let n = (count as usize).min(MAX);
    unsafe {
        N = n;
        W = w;
        H = h;
        for i in 0..n {
            POS[i * 2] = rng_f32() * w;
            POS[i * 2 + 1] = rng_f32() * h;
            // Slow ambient drift, centred on zero.
            VEL[i * 2] = (rng_f32() - 0.5) * 26.0;
            VEL[i * 2 + 1] = (rng_f32() - 0.5) * 26.0;
        }
    }
}

#[no_mangle]
pub extern "C" fn resize(w: f32, h: f32) {
    unsafe {
        // Rescale existing particles so the field reflows on resize.
        if W > 0.0 && H > 0.0 {
            let sx = w / W;
            let sy = h / H;
            for i in 0..N {
                POS[i * 2] *= sx;
                POS[i * 2 + 1] *= sy;
            }
        }
        W = w;
        H = h;
    }
}

#[no_mangle]
pub extern "C" fn pointer(x: f32, y: f32, active: u32) {
    unsafe {
        PX = x;
        PY = y;
        PACTIVE = active != 0;
    }
}

#[no_mangle]
pub extern "C" fn count() -> u32 {
    unsafe { N as u32 }
}

/// Pointer to the interleaved `[x, y]` position buffer in linear memory.
#[no_mangle]
pub extern "C" fn positions_ptr() -> *const f32 {
    core::ptr::addr_of!(POS) as *const f32
}

/// Pointer to packed `[ax, ay, bx, by, opacity]` connection records.
#[no_mangle]
pub extern "C" fn links_ptr() -> *const f32 {
    core::ptr::addr_of!(LINKS) as *const f32
}

/// Computes the proximity graph in WASM; the canvas renderer only visits visible edges.
#[no_mangle]
pub extern "C" fn build_links(radius: f32) -> u32 {
    if !radius.is_finite() || radius <= 0.0 {
        return 0;
    }
    let radius2 = radius * radius;
    let mut count = 0;
    unsafe {
        for i in 0..N {
            let ax = POS[i * 2];
            let ay = POS[i * 2 + 1];
            for j in (i + 1)..N {
                let bx = POS[j * 2];
                let by = POS[j * 2 + 1];
                let dx = ax - bx;
                let dy = ay - by;
                let distance2 = dx * dx + dy * dy;
                if distance2 < radius2 {
                    let offset = count * 5;
                    LINKS[offset] = ax;
                    LINKS[offset + 1] = ay;
                    LINKS[offset + 2] = bx;
                    LINKS[offset + 3] = by;
                    LINKS[offset + 4] = (1.0 - distance2 / radius2) * 0.42;
                    count += 1;
                }
            }
        }
    }
    count as u32
}

/// Advance the simulation by `dt` seconds.
#[no_mangle]
pub extern "C" fn step(dt: f32) {
    // Guard against tab-switch spikes.
    let dt = if dt > 0.05 { 0.05 } else { dt };
    unsafe {
        let n = N;
        let w = W;
        let h = H;
        let damp = 0.992;
        let max_speed = 64.0;
        let max_speed2 = max_speed * max_speed;
        // Pointer gravity well.
        let well_r = 170.0;
        let well_r2 = well_r * well_r;

        for i in 0..n {
            let xi = i * 2;
            let yi = xi + 1;
            let mut vx = VEL[xi];
            let mut vy = VEL[yi];

            if PACTIVE {
                let dx = PX - POS[xi];
                let dy = PY - POS[yi];
                let d2 = dx * dx + dy * dy;
                if d2 < well_r2 && d2 > 1.0 {
                    let d = sqrtf(d2);
                    // Falloff toward the edge of the well; gentle swirl.
                    let f = (1.0 - d / well_r) * 240.0;
                    let inv = 1.0 / d;
                    vx += dx * inv * f * dt;
                    vy += dy * inv * f * dt;
                    // Tangential component for a subtle orbit.
                    vx += -dy * inv * f * 0.35 * dt;
                    vy += dx * inv * f * 0.35 * dt;
                }
            }

            vx *= damp;
            vy *= damp;

            // Clamp speed.
            let sp2 = vx * vx + vy * vy;
            if sp2 > max_speed2 {
                let scale = max_speed / sqrtf(sp2);
                vx *= scale;
                vy *= scale;
            }

            let mut nx = POS[xi] + vx * dt;
            let mut ny = POS[yi] + vy * dt;

            // Soft bounce off the field edges.
            if nx < 0.0 {
                nx = 0.0;
                vx = absf(vx);
            } else if nx > w {
                nx = w;
                vx = -absf(vx);
            }
            if ny < 0.0 {
                ny = 0.0;
                vy = absf(vy);
            } else if ny > h {
                ny = h;
                vy = -absf(vy);
            }

            POS[xi] = nx;
            POS[yi] = ny;
            VEL[xi] = vx;
            VEL[yi] = vy;
        }
    }
}
