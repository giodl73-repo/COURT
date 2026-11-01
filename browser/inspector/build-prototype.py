"""Local qualification only. Does not prepare a publication-ready artifact."""
import argparse,copy,json,os,pathlib,shutil,subprocess,hashlib
p=pathlib.Path(__file__).resolve().parent
root=p.parent.parent
parser=argparse.ArgumentParser()
parser.add_argument('--bindgen',required=True)
a=parser.parse_args()
target=pathlib.Path(os.environ['CARGO_TARGET_DIR'])
assert target.is_absolute(),'CARGO_TARGET_DIR must be absolute'
assert subprocess.check_output([a.bindgen,'--version']).decode().strip()=='wasm-bindgen 0.2.127'
build_env=dict(os.environ)
cargo_home=pathlib.Path(os.environ.get('CARGO_HOME',str(pathlib.Path.home()/'.cargo'))).resolve()
build_env['RUSTFLAGS']=' '.join(['--remap-path-prefix='+str(root)+'=/court','--remap-path-prefix='+str(cargo_home)+'=/cargo'])
for arguments in [
    ['--release','--target','wasm32-unknown-unknown','--lib'],
    ['--bin','court-inspector-cli'],
]:
    subprocess.run(['cargo','+1.95.0','build','--manifest-path',str(p/'Cargo.toml'),'--locked',*arguments],cwd=root,env=build_env,check=True)
site=root/'target/court-prototype/COURT/inspector'
site.mkdir(parents=True,exist_ok=True)
for name in ['index.html','style.css','app.js','worker.js']:
    shutil.copyfile(p/'web'/name,site/name)
shutil.copyfile(p/'fixtures/snapshot.json',site/'snapshot.json')
subprocess.run([a.bindgen,'--target','web','--out-dir',str(site/'pkg'),'--out-name','court_inspector',str(target/'wasm32-unknown-unknown/release/court_inspector.wasm')],check=True)
f=json.loads((p/'fixtures/snapshot.json').read_text())
cases=[json.dumps(f),'','{}','{','null']
for actions in [[],f['actions'][:1],f['actions'][1:],f['actions']*2]:
    v=copy.deepcopy(f);v['actions']=actions;cases.append(json.dumps(v))
for role in ['Surface','Zone','Actor','Prop','Hud','Text','Media','Control','Boundary','Camera']:
    v=copy.deepcopy(f);v['scene'][0]['role']=role;cases.append(json.dumps(v))
for width in [-2147483649,-2147483648,-1,0,2147483647,2147483648,1.5]:
    v=copy.deepcopy(f);v['scene'][0]['width']=width;cases.append(json.dumps(v))
for kind in ['Legal','Unavailable','GuidedIllegal','Destructive','Diagnostic']:
    v=copy.deepcopy(f);v['actions'][0]['availability']={'kind':kind,'unexpected':True};cases.append(json.dumps(v))
for key in ['actions','scene']:
    v=copy.deepcopy(f);v[key]=[v[key][0]]*257;cases.append(json.dumps(v))
for key in ['experience_id','surface']:
    v=copy.deepcopy(f);v['metadata'][key]='Terminal' if key=='surface' else 'different';cases.append(json.dumps(v))
cases+=[' '*524289,'\ufeff'+json.dumps(f),json.dumps(f,indent=2).replace('\n','\r\n')]
cli=target/'debug'/('court-inspector-cli.exe' if os.name=='nt' else 'court-inspector-cli')
receipts=[]
for text in cases:
    result=subprocess.run([str(cli)],input=text.encode('utf-8'),capture_output=True,check=True)
    receipts.append({'input':text,'output':result.stdout.decode('utf-8').rstrip('\r\n')})
cache=p/'.cache';cache.mkdir(exist_ok=True)
(cache/'native-cases.json').write_text(json.dumps(receipts))
wasm=site/'pkg/court_inspector_bg.wasm'
receipt={'prototype_only':True,'native_cases':len(cases),'wasm_bytes':wasm.stat().st_size,'wasm_sha256':hashlib.sha256(wasm.read_bytes()).hexdigest()}
(cache/'prototype-build.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt))
