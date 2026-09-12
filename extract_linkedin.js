const fs=require('fs');
const zlib=require('zlib');
const pdf=fs.readFileSync('Ondrej Luknár _ LinkedIn.pdf');
const s=pdf.toString('latin1');

const objRe=/(\d+)\s+(\d+)\s+obj([\s\S]*?)endobj/g;
const objs=new Map();
let m;
while((m=objRe.exec(s))){
  const id=Number(m[1]);
  const gen=Number(m[2]);
  const body=m[3];
  objs.set(id,{id,gen,body});
}

function getStream(body){
  const si=body.indexOf('stream');
  if(si<0) return null;
  let st=si+6;
  if(body[st]=='\r'&&body[st+1]=='\n') st+=2;
  else if(body[st]=='\n' || body[st]=='\r') st+=1;
  const ei=body.indexOf('endstream',st);
  if(ei<0) return null;
  return body.slice(st,ei);
}
function decodeStream(raw){
  const b=Buffer.from(raw,'latin1');
  try{return zlib.inflateSync(b).toString('latin1')}catch{}
  try{return zlib.inflateRawSync(b).toString('latin1')}catch{}
  return null;
}

const decoded=new Map();
for(const [id,o] of objs){
  const raw=getStream(o.body);
  if(!raw) continue;
  const d=decodeStream(raw);
  if(d!=null) decoded.set(id,d);
}

function parseCMap(txt){
  const map=new Map();
  const bfcharRe=/(\d+)\s+beginbfchar([\s\S]*?)endbfchar/g;
  let m;
  while((m=bfcharRe.exec(txt))){
    const block=m[2];
    const lineRe=/<([0-9A-Fa-f]+)>\s*<([0-9A-Fa-f]+)>/g;
    let l;
    while((l=lineRe.exec(block))){
      const src=l[1].toUpperCase();
      const dst=l[2];
      map.set(src,hexToUnicode(dst));
    }
  }
  const bfrangeRe=/(\d+)\s+beginbfrange([\s\S]*?)endbfrange/g;
  while((m=bfrangeRe.exec(txt))){
    const block=m[2];
    const lineRe=/<([0-9A-Fa-f]+)>\s*<([0-9A-Fa-f]+)>\s*(<([0-9A-Fa-f]+)>|\[([^\]]+)\])/g;
    let l;
    while((l=lineRe.exec(block))){
      const from=parseInt(l[1],16), to=parseInt(l[2],16);
      if(l[4]){
        let base=parseInt(l[4],16);
        for(let c=from;c<=to;c++){
          const src=c.toString(16).toUpperCase().padStart(l[1].length,'0');
          map.set(src,codepointToString(base+(c-from)));
        }
      }else if(l[5]){
        const arr=[...l[5].matchAll(/<([0-9A-Fa-f]+)>/g)].map(x=>x[1]);
        for(let i=0;i<arr.length;i++){
          const c=from+i;
          if(c>to) break;
          const src=c.toString(16).toUpperCase().padStart(l[1].length,'0');
          map.set(src,hexToUnicode(arr[i]));
        }
      }
    }
  }
  return map;
}

function codepointToString(cp){
  try{return String.fromCodePoint(cp)}catch{return ''}
}
function hexToUnicode(h){
  let out='';
  for(let i=0;i<h.length;i+=4){
    const chunk=h.slice(i,i+4);
    if(chunk.length<4) break;
    out += codepointToString(parseInt(chunk,16));
  }
  return out;
}
function decodeHexByMap(hex,map){
  let out='';
  // try 4-hex (2-byte CID)
  if(hex.length%4===0){
    for(let i=0;i<hex.length;i+=4){
      const k=hex.slice(i,i+4).toUpperCase();
      out += map.get(k) ?? '';
    }
    if(out) return out;
  }
  // fallback 2-hex bytes
  for(let i=0;i<hex.length;i+=2){
    const k=hex.slice(i,i+2).toUpperCase();
    out += map.get(k) ?? String.fromCharCode(parseInt(k,16));
  }
  return out;
}

// collect cmap objects
const cmapByObj=new Map();
for(const [id,txt] of decoded){
  if(txt.includes('begincmap')) cmapByObj.set(id,parseCMap(txt));
}

// font obj -> toUnicode obj
const fontToUni=new Map();
for(const [id,o] of objs){
  if(/\/Type\s*\/Font\b/.test(o.body)){
    const tu=o.body.match(/\/ToUnicode\s+(\d+)\s+0\s+R/);
    if(tu) fontToUni.set(id,Number(tu[1]));
  }
}

// resource obj -> alias font obj
const resFonts=new Map();
for(const [id,o] of objs){
  const fm=o.body.match(/\/Font\s*<<(.*?)>>/s);
  if(!fm) continue;
  const map=new Map();
  for(const x of fm[1].matchAll(/\/(F\d+)\s+(\d+)\s+0\s+R/g)){
    map.set(x[1],Number(x[2]));
  }
  if(map.size) resFonts.set(id,map);
}

// page -> resources + contents
const pages=[];
for(const [id,o] of objs){
  if(/\/Type\s*\/Page\b/.test(o.body)){
    const r=o.body.match(/\/Resources\s+(\d+)\s+0\s+R/);
    const contents=[];
    for(const c of o.body.matchAll(/\/Contents\s+(\d+)\s+0\s+R/g)) contents.push(Number(c[1]));
    const arr=o.body.match(/\/Contents\s*\[((?:.|\n|\r)*?)\]/m);
    if(arr){ for(const c of arr[1].matchAll(/(\d+)\s+0\s+R/g)) contents.push(Number(c[1])); }
    pages.push({id,res:r?Number(r[1]):null,contents:[...new Set(contents)]});
  }
}

function decodeContent(txt,fontAliasToMap){
  let out='';
  let curFont=null;
  const tokenRe=/(\/(F\d+)\s+[\d\.]+\s+Tf|<([0-9A-Fa-f]+)>\s*Tj|\[([^\]]+)\]\s*TJ|\(([^)]*)\)\s*Tj|T\*|Td|TD|Tm|ET|BT)/g;
  let m;
  while((m=tokenRe.exec(txt))){
    if(m[2]){ curFont=m[2]; continue; }
    const cmap=curFont?fontAliasToMap.get(curFont):null;
    if(m[3]){ // hex Tj
      if(cmap) out += decodeHexByMap(m[3],cmap);
      else out += Buffer.from(m[3],'hex').toString('latin1');
      continue;
    }
    if(m[4]){ // array TJ
      for(const p of m[4].matchAll(/<([0-9A-Fa-f]+)>|\(([^)]*)\)/g)){
        if(p[1]) out += cmap?decodeHexByMap(p[1],cmap):Buffer.from(p[1],'hex').toString('latin1');
        if(p[2]) out += p[2];
      }
      continue;
    }
    if(m[5]){ out += m[5]; continue; }
    // new line hints
    if(m[0]==='ET' || m[0]==='T*') out += '\n';
  }
  return out;
}

let full='';
for(const p of pages){
  const fmap=new Map();
  const rmap=p.res?resFonts.get(p.res):null;
  if(rmap){
    for(const [alias,fontObj] of rmap){
      const tu=fontToUni.get(fontObj);
      if(tu && cmapByObj.get(tu)) fmap.set(alias,cmapByObj.get(tu));
    }
  }
  full += `\n\n===== PAGE ${p.id} =====\n`;
  for(const c of p.contents){
    const t=decoded.get(c);
    if(!t) continue;
    const d=decodeContent(t,fmap).replace(/[\u0000-\u0008\u000B\u000C\u000E-\u001F]/g,'');
    full += d + '\n';
  }
}

fs.writeFileSync('linkedin_extracted.txt',full,'utf8');
console.log('objects',objs.size,'decodedStreams',decoded.size,'pages',pages.length,'cmap',cmapByObj.size);
