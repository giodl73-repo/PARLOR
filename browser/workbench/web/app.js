const $=id=>document.getElementById(id),enc=new TextEncoder();let revision=0,generation=0,worker=null,pending=null,timer=null,readToken=0,original=null,executed=null;
const status=s=>$("status").textContent=s,bytes=s=>enc.encode(s).length;
function download(data,name,type="text/plain;charset=utf-8"){const url=URL.createObjectURL(new Blob([data],{type})),a=document.createElement("a");a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
function stop(){clearTimeout(timer);timer=null;generation++;worker?.terminate();worker=null;pending=null;$("cancel").disabled=true;}
function changed(){revision++;readToken++;stop();executed=null;$("results").hidden=true;$("empty").hidden=false;$("empty").textContent="Run the current request to inspect its result.";$("export-request").disabled=true;$("export-report").disabled=true;}
function failed(s){stop();executed=null;$("results").hidden=true;$("export-request").disabled=true;$("export-report").disabled=true;status(s+" Current input and opened original are retained; source/build routes remain available.");}
function render(data,raw,ms){
 $("results").hidden=false;$("empty").hidden=true;$("game").textContent=data.game.name+" · "+data.game.players+" native players";$("boundary").textContent=data.boundary;$("raw").textContent=raw;$("metrics").replaceChildren();$("records").replaceChildren();$("records-head").replaceChildren();$("benchmarks").replaceChildren();
 let headers=[],rows=[];
 if(data.perft){$("summary").textContent=`Native perft depth ${data.perft.depth}: ${data.perft.nodes} leaves. ${data.perft.evidence}`;const p=document.createElement("p");p.textContent=`Preflight visits: ${data.perft.preflight_visited}. The admission walk is not the reported count.`;$("metrics").append(p);
  if(data.game.id==="chess"){headers=["Native legal UCI move"];rows=data.legal_moves.map(x=>[x]);}else{headers=["Native legal path","Captured squares"];rows=data.legal_moves.map(x=>[JSON.stringify(x.path),JSON.stringify(x.captures)]);}
 }else if(data.game.id==="backgammon"){
  $("summary").textContent=`Native pip counts: white ${data.pip_count.white}, black ${data.pip_count.black}. ${data.legal_plays.length} distinct native successors for ${data.query_player} under ${data.dice.join(" + ")}.`;
  const p=document.createElement("p");p.textContent=data.pip_count.evidence+". Dice model mean: "+data.dice_model.expected_pips+". "+data.dice_model.evidence;$("metrics").append(p);headers=["Native selection","Hits","White pips","Black pips","Native resulting-board debug"];rows=data.legal_plays.map(x=>[x.selection,x.hits,x.white_pips,x.black_pips,x.native_debug]);
 }else{
  $("summary").textContent=`Native ${data.size}×${data.size} Go / Baduk: ${data.legal_move_count} legal moves for ${data.query_player}; area black ${data.area_score.black}, white ${data.area_score.white}.`;
  const p=document.createElement("p");p.textContent=data.area_score.evidence+". Nominal native to-move: "+data.nominal_to_move+". Legal query coordinates: "+JSON.stringify(data.legal_moves);$("metrics").append(p);headers=["Row","Column","Explicit color","Native captured coordinates"];rows=data.placements.map(x=>[x.row,x.column,x.color,JSON.stringify(x.captured)]);
 }
 $("records-caption").textContent=data.game.id==="go"?"Native placement events":"Native legal continuations";const tr=document.createElement("tr");for(const h of headers){const th=document.createElement("th");th.scope="col";th.textContent=h;tr.append(th);}$("records-head").append(tr);
 for(const row of rows){const tr=document.createElement("tr");for(const text of row){const td=document.createElement("td");td.textContent=String(text);tr.append(td);}$("records").append(tr);}
 $("benchmark-box").hidden=!data.retained_benchmarks;for(const b of data.retained_benchmarks||[]){const tr=document.createElement("tr");for(const x of [b.name,b.depth,b.expected_nodes,b.source,b.matches_request?"Exact input/depth match; compare with current native count":"Not selected by this exact request"]){const td=document.createElement("td");td.textContent=String(x);tr.append(td);}$("benchmarks").append(tr);}
 $("timing").textContent=`Worker computation and serialization: ${Number(ms).toFixed(1)} ms. No comparative speed claim.`;$("export-request").disabled=false;$("export-report").disabled=false;
}
function valid(r,input){
 if(!r||typeof r!=="object")return false;if(r.kind!=="report")return ["native_error","input_error","browser_refusal"].includes(r.kind)&&typeof r.error==="string";
 const d=r.data;try{if(d?.game?.id!==JSON.parse(input).game)return false;}catch{return false;}if(!d||!d.game||typeof d.game.name!=="string"||!Number.isInteger(d.game.players)||typeof d.boundary!=="string")return false;
 if(d.game.id==="chess"||d.game.id==="checkers")return d.perft&&typeof d.perft.nodes==="string"&&typeof d.perft.evidence==="string"&&Array.isArray(d.legal_moves)&&Array.isArray(d.retained_benchmarks)&&d.legal_moves.every(x=>d.game.id==="chess"?typeof x==="string":x&&Array.isArray(x.path)&&Array.isArray(x.captures));
 if(d.game.id==="backgammon")return d.pip_count&&Array.isArray(d.legal_plays)&&d.legal_plays.every(x=>x&&typeof x.native_debug==="string")&&Array.isArray(d.dice)&&d.dice_model;
 return d.game.id==="go"&&d.area_score&&Array.isArray(d.legal_moves)&&Array.isArray(d.placements)&&d.placements.every(x=>x&&Array.isArray(x.captured));
}
async function run(){
 stop();readToken++;executed=null;$("export-request").disabled=true;$("export-report").disabled=true;$("results").hidden=true;$("empty").hidden=false;
 const input=$("input").value;if(bytes(input)>262144){failed("Browser refusal: current request exceeds256KiB.");return;}
 const id=++revision,g=++generation,rev=revision;pending={id,generation:g,rev,phase:"starting"};$("cancel").disabled=false;status("Starting Rust worker…");const alive=()=>pending&&revision===rev&&generation===g&&pending.id===id;
 try{worker=new Worker(new URL("./worker.js",import.meta.url),{type:"module"});}catch(e){failed("Worker unavailable: "+e);return;}
 timer=setTimeout(()=>{if(alive())failed("Startup deadline exceeded (12 seconds).");},12000);worker.onerror=()=>{if(alive())failed("Engine crashed; run the request again to restart.");};
 worker.onmessage=async({data})=>{
  if(!alive()||data.id!==id||data.generation!==g)return;
  if(data.type==="ready"){
   if(pending.phase==="running")return;pending.phase="running";clearTimeout(timer);timer=setTimeout(()=>{if(alive())failed("Computation deadline exceeded (10 seconds).");},10000);status("Running native kernel…");worker.postMessage({type:"run",id,generation:g,input});
  }else if(data.type==="fatal")failed("Engine unavailable: "+data.error);
  else if(data.type==="report"){
   try{if(pending.phase!=="running"||typeof data.raw!=="string"||bytes(data.raw)>2097152)throw Error("invalid envelope");const r=JSON.parse(data.raw),hash=[...new Uint8Array(await crypto.subtle.digest("SHA-256",enc.encode(input)))].map(x=>x.toString(16).padStart(2,"0")).join("");if(!alive())return;if(r.request_sha256!==hash||!valid(r,input))throw Error("invalid report/hash");if(r.kind!=="report"){failed(r.kind+": "+r.error);return;}render(r.data,data.raw,data.elapsedMs);executed={input,raw:data.raw};stop();status("Native result ready. Evidence labels and model boundaries remain visible.");}catch(e){if(alive())failed("Malformed engine report: "+e);}
  }
 };
 worker.postMessage({type:"init",id,generation:g});
}
$("input").oninput=()=>{changed();status("Input changed. Run the current request.");};$("run").onclick=run;$("cancel").onclick=()=>{changed();status("Computation cancelled; current input retained.");};
$("original").onclick=()=>{if(original)download(original,"opened-request-original.json","application/json");};$("current").onclick=()=>download($("input").value,"current-request.json","application/json");$("lf").onclick=()=>{changed();$("input").value=$("input").value.replace(/^\uFEFF/,"").replace(/\r\n?/g,"\n");status("Explicit LF editor copy selected. Opened original unchanged.");};$("export-request").onclick=()=>{if(executed)download(executed.input,"executed-request.json","application/json");};$("export-report").onclick=()=>{if(executed)download(executed.raw,"native-report.json","application/json");};
async function read(file){if(!file)return;changed();const token=++readToken;if(file.size>262144){status("Browser refusal: whole file exceeds256KiB. Prior editor/original retained.");return;}let timeout;try{const b=await Promise.race([file.arrayBuffer(),new Promise((_,reject)=>timeout=setTimeout(()=>reject(Error("Read deadline exceeded (5 seconds)")),5000))]);if(token!==readToken)return;const raw=new Uint8Array(b);if(raw.length>262144)throw Error("Whole file exceeds bound");const text=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(raw);original=raw;$("input").value=text;$("origin").textContent=`Opened ${file.name}: ${raw.length} original bytes.`;$("original").disabled=false;status("Request opened. Run the native kernel; use LF copy if a leading BOM needs removal.");}catch(e){if(token===readToken)status("Could not read request: "+e+". Prior editor/original retained.");}finally{clearTimeout(timeout);}}
$("file").onchange=e=>read(e.target.files[0]);$("input").ondragover=e=>e.preventDefault();$("input").ondrop=e=>{e.preventDefault();read(e.dataTransfer.files[0]);};
async function example(auto=false){changed();const token=++readToken;status("Loading example request…");try{const r=await fetch(new URL("./examples.json",import.meta.url));if(!r.ok)throw Error(r.status);const all=await r.json();if(token!==readToken)return;$("input").value=JSON.stringify(all[$("example").value],null,2);if(auto)run();else status("Example loaded. Run the native kernel.");}catch(e){if(token===readToken)status("Example unavailable: "+e);}}
$("load-example").onclick=()=>example();$("example").onchange=()=>{changed();status("Example selection changed. Load it to replace the current input.");};example(true);
