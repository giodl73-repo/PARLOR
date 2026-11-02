import init,{evaluate} from "./pkg/parlor_browser_workbench.js";
let identity;
self.onmessage=async({data})=>{
 if(data.type==="init"){
  identity={id:data.id,generation:data.generation};
  try{await init();self.postMessage({type:"ready",...identity});}catch(error){self.postMessage({type:"fatal",...identity,error:String(error)});}
 }else if(data.type==="run"&&identity&&data.id===identity.id&&data.generation===identity.generation){
  try{const start=performance.now();const raw=evaluate(data.input);self.postMessage({type:"report",...identity,raw,elapsedMs:performance.now()-start});}catch(error){self.postMessage({type:"fatal",...identity,error:String(error)});}
 }
};
