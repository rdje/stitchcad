"""Independent census fixtures and in-memory actual-guard faults; no source rewrites."""
from pathlib import Path
import os
import runpy
import sys
import subprocess
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'scripts/handoff_census.py'
RAW = SOURCE.read_bytes()
R = str(ROOT / 'target/handoff fixture')
NODE = str(ROOT / 'target/runtime/cua_node/bin/node')
SANDBOX = str(ROOT / 'target/runtime/codex-cli/CodexCLI.app/Contents/MacOS/codex')
KERNEL = NODE + ' --experimental-vm-modules worker/kernel.js --session-id abc-123 --working-dir ' + R
WRAPPER = SANDBOX + ' sandbox -c default_permissions="node_repl" -c permissions.node_repl={filesystem={}} -- ' + KERNEL
BASE_PS = '90 1 01:00 harness\n100 90 00:01 census\n'
BASE_LSOF = 'p100\nf0\ntPIPE\nn\n'
CASES = []


def case(name, ps=BASE_PS, lsof=BASE_LSOF, expected=(), advisory=(), idle=False, refusal=None,
         status_ps=0, status_lsof=0, deny=None):
    CASES.append((name, ps, lsof, expected, advisory, idle, refusal, status_ps, status_lsof, deny))


def file(pid, fd='3', path=None, kind='REG'):
    return 'p%d\nf%s\nt%s\nn%s\n' % (pid, fd, kind, path or R + '/tracked.md')


case('empty work')
case('inherited cwd', ps=BASE_PS+'200 90 01:00 caffeine\n', lsof=BASE_LSOF+file(200,'cwd',R,'DIR'), advisory=(200,))
case('real handle outside cwd', ps=BASE_PS+'200 90 01:00 ordinary\n', lsof=BASE_LSOF+file(200), expected=(200,))
case('checkout argument', ps=BASE_PS+'200 90 01:00 python '+R+'\n', expected=(200,))
case('script names do not exempt handles', ps=BASE_PS+'200 90 01:00 check_handoff.sh\n', lsof=BASE_LSOF+file(200), expected=(200,))
case('caller ancestor handles excluded', lsof=BASE_LSOF+file(90))
case('new PID after ps still blocks', lsof=BASE_LSOF+file(500), expected=(500,))
case('other root prefix handle', ps=BASE_PS+'200 90 01:00 ordinary\n', lsof=BASE_LSOF+file(200,path=R+'-other/tracked.md'))
case('outside file', ps=BASE_PS+'200 90 01:00 ordinary\n', lsof=BASE_LSOF+file(200,path=str(ROOT.parent/'other/file')))
PAIR_PS = BASE_PS+'200 90 01:00 '+WRAPPER+'\n300 200 01:00 '+KERNEL+'\n'
PAIR_LSOF = BASE_LSOF+file(200,'cwd',R,'DIR')+file(300,'cwd',R,'DIR')
case('CUA needs attestation', ps=PAIR_PS,lsof=PAIR_LSOF,expected=(200,300))
case('attested idle pair', ps=PAIR_PS,lsof=PAIR_LSOF,idle=True,advisory=(200,300))
case('kernel file handle beats attestation', ps=PAIR_PS,lsof=BASE_LSOF+file(300),idle=True,expected=(200,300))
case('wrapper file handle beats attestation', ps=PAIR_PS,lsof=BASE_LSOF+file(200),idle=True,expected=(200,300))
WORKER = NODE+' --experimental-vm-modules worker/trusted-worker.js '+R
WORKER_PS = PAIR_PS.replace(KERNEL,WORKER)
case('attested worker',ps=WORKER_PS,lsof=PAIR_LSOF,idle=True,advisory=(200,300))
case('extra executable args',ps=PAIR_PS.replace(KERNEL,KERNEL+' --eval work'),lsof=PAIR_LSOF,idle=True,expected=(200,300))
case('unpaired kernel',ps=BASE_PS+'300 90 01:00 '+KERNEL+'\n',lsof=BASE_LSOF+file(300,'cwd',R,'DIR'),idle=True,expected=(300,))
case('wrong wrapper',ps=PAIR_PS.replace('codex sandbox ','codex execute '),lsof=PAIR_LSOF,idle=True,expected=(200,300))
case('different child launch',ps=PAIR_PS.replace('300 200 01:00 '+KERNEL,'300 200 01:00 '+WORKER),lsof=PAIR_LSOF,idle=True,expected=(200,300))
case('missing permissions marker',ps=PAIR_PS.replace('permissions.node_repl=','permissions.other='),lsof=PAIR_LSOF,idle=True,expected=(200,300))
case('ps denied',deny='ps',refusal='permission')
case('lsof denied',deny='lsof',refusal='permission')
case('ps nonzero despite output',status_ps=1,refusal='ps unavailable')
case('lsof nonzero despite output',status_lsof=1,refusal='lsof unavailable')
case('ps empty',ps=' ',refusal='ps returned empty')
case('lsof empty',lsof='\n',refusal='lsof returned empty')
case('ps malformed',ps=BASE_PS+'oops\n',refusal='malformed ps')
case('duplicate ps pid',ps=BASE_PS+'100 90 00:00 duplicate\n',refusal='ps identity')
case('missing caller',ps='90 1 00:00 harness\n',refusal='caller ancestry')
case('missing ancestor',ps='100 90 00:00 census\n',refusal='caller ancestry')
case('cycle ancestry',ps='90 100 00:00 harness\n100 90 00:00 census\n',refusal='caller ancestry')
case('missing lsof caller',lsof=file(200),refusal='caller missing from lsof')
case('lsof bad pid',lsof='pX\n',refusal='lsof identity')
case('duplicate lsof pid',lsof=BASE_LSOF+BASE_LSOF,refusal='lsof identity')
case('file without process',lsof='f3\ntREG\nnx\n',refusal='file field')
case('name without file',lsof='p100\nnx\n',refusal='name field')
case('missing type',lsof='p100\nf3\nnx\n',refusal='name field')
case('unknown field',lsof=BASE_LSOF+'Zx\n',refusal='unexpected lsof')
case('truncated process',lsof=BASE_LSOF+'p200\n',refusal='truncated lsof process')
case('truncated file',lsof='p100\nf3\ntREG\n',refusal='truncated lsof process')
case('unnamed regular file',lsof='p100\nf3\ntREG\nn\n',refusal='unnamed filesystem')
case('unnamed unknown type',lsof='p100\nf3\ntFUTURE\nn\n',refusal='unnamed filesystem')
case('unnamed NPOLICY',lsof=BASE_LSOF+'f4\ntNPOLICY\nn\n')
case('unnamed NEXUS',lsof=BASE_LSOF+'f4\ntNEXUS\nn\n')


def verify(ns):
    for name,ps,lsof,expected,advisory,idle,refusal,status_ps,status_lsof,deny in CASES:
        calls=[]
        def run(command, **kwargs):
            calls.append(command[0])
            assert kwargs == {'capture_output':True,'text':True}
            if command[0]==deny: raise PermissionError('permission denied')
            return SimpleNamespace(returncode=status_ps if command[0]=='ps' else status_lsof,
                                   stdout=ps if command[0]=='ps' else lsof)
        try:
            blocks,notes=ns['census'](R,100,999,idle,run)
        except (ns['Refusal'],OSError) as e:
            assert refusal and refusal in str(e),(name,'unexpected refusal',str(e))
        else:
            assert refusal is None,(name,'failed to refuse',refusal)
            assert tuple(b[0] for b in blocks)==expected,(name,'blocking',blocks,expected)
            assert tuple(n[0] for n in notes)==advisory,(name,'advisory',notes,advisory)
            assert calls==['ps','lsof'],(name,'single primary snapshots',calls)


ns=runpy.run_path(str(SOURCE));verify(ns)
FAULTS=[
 ('nonzero evidence','if result.returncode != 0:','if False:'),
 ('empty evidence','if not result.stdout.strip():','if False:'),
 ('caller handle proof','if caller not in seen:','if False:'),
 ('actual file blocking','if count > 0 or (names_root and pid not in idle):','if names_root and pid not in idle:'),
 ('root argument blocking','if count > 0 or (names_root and pid not in idle):','if count > 0:'),
 ('idle attestation','if idle_cua else set()','if True else set()'),
 ('parent handle priority','and not handles.get(parent.pid, 0)','and True'),
 ('child handle priority','and not handles.get(child.pid, 0)','and True'),
 ('wrapper identity','if (shape and separator','if (True and separator'),
 ('matching launch','and tail == child.command','and True'),
 ('permissions marker',"and 'permissions.node_repl=' in launcher",'and True'),
 ('unnamed type proof',"if not value and kind not in ('PIPE', 'NPOLICY', 'NEXUS'):",'if False:'),
 ('truncation proof','if not named or complete != seen:','if False:'),
]
for name,old,new in FAULTS:
    assert RAW.decode().count(old)==1,(name,'fault target')
    altered={'__name__':'handoff_fault'}
    exec(compile(RAW.decode().replace(old,new),str(SOURCE),'exec'),altered)
    try:verify(altered)
    except AssertionError:pass
    else:raise AssertionError('actual guard fault escaped: '+name)
assert SOURCE.read_bytes()==RAW
print('handoff contract: %d fixtures / %d actual assertion reds / producer unchanged' % (len(CASES),len(FAULTS)))

if sys.argv[1:] == ['--real']:
    (ROOT / 'target').mkdir(exist_ok=True)
    entry = ['bash', 'scripts/check_handoff.sh', '--idle-cua']
    baseline = subprocess.run(entry, cwd=ROOT, capture_output=True, text=True)
    assert baseline.returncode == 0, ('real baseline needs no owned jobs', baseline.stdout, baseline.stderr)
    child = subprocess.Popen([sys.executable, '-I', '-B', '-c',
        "import os,sys; f=open('target/handoff-real-handle.txt','w'); print(os.getpid(),flush=True); sys.stdin.read()"],
        cwd=ROOT, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        pid = int(child.stdout.readline())
        result = subprocess.run(entry, cwd=ROOT, capture_output=True, text=True)
        assert result.returncode == 1 and ('PID %d: repo handle' % pid) in result.stdout
        print('real held-file census: blocks PID %d, exit1' % pid)
    finally:
        child.stdin.close()
        child.wait(timeout=10)
    restored = subprocess.run(entry, cwd=ROOT, capture_output=True, text=True)
    assert child.returncode == 0 and restored.returncode == 0, (restored.stdout, restored.stderr)
    print('real restored census: holder terminal0 / handoff OK0')
elif sys.argv[1:]:
    raise SystemExit('usage: contract.py [--real]')
