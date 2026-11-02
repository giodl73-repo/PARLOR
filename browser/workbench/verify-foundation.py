"""Protect reconciled native kernels, sources and both retained FERRIS controls."""
import json,pathlib,subprocess
R=pathlib.Path(__file__).resolve().parents[2];ENTRY='aea809441e907a4f9de764ae2466c4305a13496b'
paths=['Cargo.toml','crates','.roles','.pitfall','PRODUCT_PLAN.md','tools/ferris-contract','tools/ferris-go','.github/workflows/ferris-contract.yml','.github/workflows/ferris-go.yml','context/waves/2026-08-ferris-adoption','context/waves/2026-08-ferris-go-shadow']
for args in [['diff','--name-only',ENTRY,'--',*paths],['diff','--cached','--name-only',ENTRY,'--',*paths]]:assert not subprocess.check_output(['git',*args],cwd=R).strip(),'Protected kernel/policy/source/control change'
old=subprocess.check_output(['git','show',ENTRY+':README.md'],cwd=R).decode();assert(R/'README.md').read_text(encoding='utf-8').startswith(old),'README front door is append-only'
assert not subprocess.check_output(['git','diff','--name-only','770ee14659cc82bda9ce70b1ddb47a9cc0cb6df7','--','crates'],cwd=R).strip(),'Native public kernels must remain unchanged'
print(json.dumps({'canonical_entry':'66ddd112827aeaff525daa0fe259f74222951bc6','public_entry':'770ee14659cc82bda9ce70b1ddb47a9cc0cb6df7','reconciled_entry':ENTRY,'native_kernels_shared_api_cli_tests_benchmarks':'unchanged from public entry','roles_policy_ferris_owner_controls':'unchanged from reconciled entry','root_lock':'new committed reproducible lock; six native packages, no external runtime deps'}))
