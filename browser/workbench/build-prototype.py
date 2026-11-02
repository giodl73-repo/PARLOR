"""Stage synthetic local prototype; native kernels and original CLI are the oracles."""
import argparse,hashlib,json,os,pathlib,shutil,subprocess
p=argparse.ArgumentParser();p.add_argument('--bindgen',required=True,type=pathlib.Path);a=p.parse_args();B=pathlib.Path(__file__).resolve().parent;R=B.parents[1];C=B/'.cache';C.mkdir(exist_ok=True);env=os.environ.copy();target=pathlib.Path(env.get('CARGO_TARGET_DIR',str(R/'target/cargo'))).resolve();env['CARGO_TARGET_DIR']=str(target);env['CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS']='-C link-arg=--max-memory=67108864'
def run(args,**kw):return subprocess.run([str(x)for x in args],cwd=R,env=env,check=True,**kw)
assert subprocess.check_output([str(a.bindgen.resolve()),'--version']).decode().strip()=='wasm-bindgen 0.2.127'
for args in [[],['--target','wasm32-unknown-unknown']]:run(['cargo','+1.95.0','build','--release','--locked','--manifest-path',B/'Cargo.toml',*args])
run(['cargo','+1.95.0','build','--release','--locked','-p','parlor-cli'])
S=R/'target/kernel-prototype/PARLOR';assert S.resolve()==R/'target/kernel-prototype/PARLOR'
if S.exists():shutil.rmtree(S)
shutil.copytree(B/'web',S);run([a.bindgen.resolve(),target/'wasm32-unknown-unknown/release/parlor_browser_workbench.wasm','--target','web','--out-dir',S/'pkg'])
for name in ['parlor_browser_workbench.d.ts','parlor_browser_workbench_bg.wasm.d.ts']:(S/'pkg'/name).unlink()
for name in ['BUILD.md','MODEL.md']:shutil.copyfile(B/name,S/name)
suffix='.exe'if os.name=='nt'else'';binary=target/('release/parlor-browser-control'+suffix);cli=target/('release/parlor'+suffix)
def text(v):return json.dumps(v,ensure_ascii=False,separators=(',',':'))
def eval(q):
 raw=q if isinstance(q,str)else text(q);lines=run([binary],input=(raw+'\n').encode(),capture_output=True).stdout.decode().splitlines();assert len(lines)==1;return raw,json.loads(lines[0])
def cli_text(args):return run([cli,*args],capture_output=True).stdout.decode().strip()
cases=[];cli_checks=[]
def add(name,q,kind='report'):
 raw,r=eval(q);assert r['kind']==kind,(name,r);assert eval(q)[1]==r;assert r['request_sha256']==hashlib.sha256(raw.encode()).hexdigest();cases.append({'request':raw,'response':r});return r
examples=json.loads((B/'web/examples.json').read_bytes())
for name,q in examples.items():add(name,q)
for name in ['chess','kiwipete']:
 for d in range(4):
  q=dict(examples[name],depth=d);r=add(f'{name} depth{d}',q);args=['chess','perft','--depth',str(d)]+(['--fen',q['fen']]if'fen'in q else[]);value=cli_text(args);assert value==r['data']['perft']['nodes'];cli_checks.append({'command':args,'stdout':value})
  args=['chess','moves']+(['--fen',q['fen']]if'fen'in q else[]);lines=cli_text(args).splitlines();assert lines[:-1]==r['data']['legal_moves'] and lines[-1]==f"{len(lines)-1} legal moves";cli_checks.append({'command':args,'stdout':'\n'.join(lines)})
for d in range(4):
 q={'game':'checkers','depth':d};r=add(f'checkers depth{d}',q);args=['checkers','perft','--depth',str(d)];value=cli_text(args);assert value==r['data']['perft']['nodes'];cli_checks.append({'command':args,'stdout':value})
for size in [1,5,9,19]:
 q={'game':'go','size':size,'player':'black'};r=add(f'Go empty{size}',q)
 for command,field in [('points','point_count'),('legal','legal_move_count')]:
  args=['go',command,'--size',str(size)];value=cli_text(args);assert int(value)==r['data'][field];cli_checks.append({'command':args,'stdout':value})
for a1,b1 in [(1,1),(1,2),(2,5),(6,6)]:
 q={'game':'backgammon','player':'white','dice':[a1,b1]};r=add(f'roll{a1}-{b1}',q);args=['backgammon','roll',str(a1),str(b1)];value=cli_text(args);assert value==f"{len(r['data']['legal_plays'])} distinct legal plays for {a1}-{b1}";cli_checks.append({'command':args,'stdout':value})
add('own chess continuation',{'game':'chess','moves':['e2e4','e7e5'],'depth':2});add('illegal chess move',{'game':'chess','moves':['e2e5']},'native_error')
check=add('checkers path seed',{'game':'checkers'});path=check['data']['legal_moves'][0]['path'];add('own checkers continuation',{'game':'checkers','paths':[path],'depth':2});add('illegal checkers path',{'game':'checkers','paths':[[0,63]]},'native_error')
add('own backgammon successor',{'game':'backgammon','turns':[{'player':'white','dice':[1,2],'selection':0}],'player':'black','dice':[3,4]});add('invalid backgammon successor',{'game':'backgammon','turns':[{'player':'white','dice':[1,2],'selection':4294967295}],'player':'black','dice':[3,4]},'native_error')
add('Go explicit same color',{'game':'go','size':5,'placements':[{'row':0,'column':0,'color':'black'},{'row':0,'column':1,'color':'black'}],'player':'white'})
add('Go occupied',{'game':'go','size':5,'placements':[{'row':0,'column':0,'color':'black'},{'row':0,'column':0,'color':'white'}],'player':'black'},'native_error')
add('Go offboard',{'game':'go','size':5,'placements':[{'row':4294967295,'column':0,'color':'black'}],'player':'black'},'native_error')
for fen in ['8/8','8/8/8/8/8/8/8/8 w - -','8/8/8/8/8/8/8/8 invalid - -','8/8/8/8/8/8/8/8 w - z9']:
 raw,r=eval({'game':'chess','fen':fen});add('native FEN boundary',{'game':'chess','fen':fen},r['kind']);result=subprocess.run([str(cli),'chess','moves','--fen',fen],cwd=R,capture_output=True);assert(result.returncode==0)==(r['kind']=='report')
 if result.returncode:assert result.stderr.decode().strip()=='invalid FEN: '+r['error']
 cli_checks.append({'command':['chess','moves','--fen',fen],'exit':result.returncode,'stdout':result.stdout.decode(),'stderr':result.stderr.decode()})
add('native ignored FEN clocks',{'game':'chess','fen':examples['kiwipete']['fen']+' extra-field','depth':1})
for q in [{'game':'chess','depth':4},{'game':'go','size':20,'player':'black'},{'game':'go','size':0,'player':'black'},{'game':'backgammon','dice':[0,7],'player':'white'},{'game':'backgammon','dice':[1,2],'player':'other'},{'game':'chess','fen':'x'*4097},{'game':'chess','moves':['e2e4']*129}]:add('browser resource refusal',q,'browser_refusal')
for raw in ['{','null','{}','{"game":"chess","game":"go"}','{"game":"chess","unexpected":1}',text({'game':'chess'})+' {}',text({'game':'go','size':5,'player':'black','placements':[{'row':4294967296,'column':0,'color':'black'}]})]:add('strict transport rejection',raw,'input_error')
(C/'native-cases.json').write_text(json.dumps(cases,ensure_ascii=False,indent=2)+'\n',encoding='utf-8');(C/'native-controls.json').write_text(json.dumps({'cases':len(cases),'each_repeated':2,'independent_native_cli_controls':len(cli_checks),'cli_checks':cli_checks},indent=2)+'\n',encoding='utf-8');print(json.dumps({'cases':len(cases),'direct_cli_controls':len(cli_checks),'staged':str(S)}))
