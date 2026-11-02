"""Named browser admission controls. Synthetic stress FEN is not lawful chess."""
import json
def controls():
 rows=[]
 def add(name,q,kind):rows.append({'name':name,'request':q,'expected_kind':kind})
 start='rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
 for n in [4096,4097]:add(f'FEN {n} bytes',{'game':'chess','fen':start+' '*(n-len(start)),'depth':0},'report'if n==4096 else'browser_refusal')
 for n in [4096,4098]:
  s=start+' α';s+='α'*((n-len(s.encode()))//2);s+=' '*(n-len(s.encode()));assert len(s.encode())==n;add(f'UTF8 FEN {n} bytes',{'game':'chess','fen':s,'depth':0},'report'if n==4096 else'browser_refusal')
 cycle=['g1f3','g8f6','f3g1','f6g8']*32
 add('maximum128 legal chess moves',{'game':'chess','moves':cycle,'depth':0},'report');add('129 chess moves',{'game':'chess','moves':cycle+['g1f3'],'depth':0},'browser_refusal')
 for n in [32,33]:add(f'checkers path length{n}',{'game':'checkers','paths':[[0]*n]},'native_error'if n==32 else'browser_refusal')
 add('checkers square64',{'game':'checkers','paths':[[64,0]]},'browser_refusal')
 for n in [128,129]:add(f'checkers entries{n}',{'game':'checkers','paths':[[0,63]]*n},'native_error'if n==128 else'browser_refusal')
 for n in [16,17]:add(f'backgammon selections{n}',{'game':'backgammon','turns':[{'player':'white'if i%2==0 else'black','dice':[1,2],'selection':0}for i in range(n)],'player':'black','dice':[3,4]},'report'if n==16 else'browser_refusal')
 placements=[{'row':i//19,'column':i%19,'color':'black'}for i in range(128)]
 add('maximum Go19 with128 explicit placements',{'game':'go','size':19,'placements':placements,'player':'white'},'report');add('Go129 placements',{'game':'go','size':19,'placements':placements+[{'row':18,'column':18,'color':'white'}],'player':'white'},'browser_refusal')
 add('maximum admitted Kiwipete preflight99950',{'game':'chess','fen':'r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1','depth':3},'report')
 add('actual100000-node budget refusal; synthetic no-kings stress FEN',{'game':'chess','fen':'qqqqqqqq/8/8/8/8/8/8/QQQQQQQQ w - - 0 1','depth':3},'browser_refusal')
 for d in [0,3,4]:add(f'perft depth{d}',{'game':'checkers','depth':d},'report'if d<=3 else'browser_refusal')
 raw=json.dumps({'game':'chess','depth':0},separators=(',',':'));raw+=' '*(262144-len(raw.encode()));add('request256KiB',raw,'report');add('request256KiB plus1',raw+' ','browser_refusal')
 return rows
