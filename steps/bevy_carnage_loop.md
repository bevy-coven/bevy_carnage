# Where bevy_carnage_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_carnage_loop.tsv`](bevy_carnage_loop.tsv). Updated 2026-11-01 13:08Z.

## What the last passes shipped

From [`bevy_carnage_loop-audit.tsv`](bevy_carnage_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 0 | 2026-11-01T00:31Z | 2 | 0 | 0 | - | 0 | 0 | - |
| 15 | 2026-11-01T03:37Z | 10 | 0 | 4 | v0.8.2 | 9 | 1 | - |
| 16 | 2026-11-01T07:10Z | 10 | 0 | 6 | v0.8.3 | 11 | 2 | - |
| 17 | 2026-11-01T09:53Z | 8 | 0 | 4 | v0.8.4 | 9 | 1 | - |
| 18 | 2026-11-01T13:08Z | 8 | 0 | 3 | v0.8.5 | 9 | 2 | - |

## The last 12 passes

```text
12 pass(es), 5393 steps, 38.2 h of pass time

category                       hours  share  steps  mean s
wait: gate                     11.07  29.0%     37  1077.2
wait: build                     7.26  19.0%     60   435.4
model                           6.45  16.9%   2472     9.4
wait: other                     3.92  10.3%     53   266.4
wait: tests                     3.07   8.0%     47   234.9
wait: clip                      2.54   6.6%     54   169.3
shell                           1.40   3.7%    831     6.1
cargo test                      0.78   2.0%    114    24.6
wait: subagent                  0.38   1.0%      2   692.1
wait: release                   0.34   0.9%      8   151.1
cargo build                     0.31   0.8%     80    13.9
web                             0.09   0.2%     29    11.3
edit files                      0.09   0.2%    448     0.7
remote wait                     0.09   0.2%     12    26.2
read files                      0.07   0.2%    662     0.4

phase                          hours  share  steps  mean s
orient                         30.59  80.0%   4152    26.5
build                           3.36   8.8%    699    17.3
gate                            2.24   5.9%    142    56.9
prove                           0.67   1.8%    111    21.8
release                         0.61   1.6%     51    42.9
research                        0.38   1.0%    117    11.7
ship                            0.35   0.9%     76    16.4
log                             0.02   0.0%      8     7.6

slowest single steps
  pass   18    5563s  bg    shell                bg_4 completed: tools/webgl2/check.sh . 12 2>&1 | tail -25
  pass   12    1862s  tool  wait: gate           Waiting for merged-tree gate
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    8    1800s  tool  wait: gate           Waiting for the gate
  pass    9    1800s  tool  wait: gate           Waiting for the gate sequence
  pass   16    1800s  tool  wait: gate           Waiting for gate or harness
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: tests          Waiting for test compile
  pass    8    1800s  tool  wait: other          Waiting for the full QA round
  pass    8    1800s  tool  wait: gate           Waiting for the gate
```

## Every pass recorded

```text
18 pass(es), 5501 steps, 42.3 h of pass time

category                       hours  share  steps  mean s
wait: build                    11.23  26.5%     68   594.5
wait: gate                     11.07  26.2%     37  1077.2
model                           6.54  15.5%   2526     9.3
wait: other                     3.92   9.3%     53   266.4
wait: tests                     3.07   7.3%     47   234.9
wait: clip                      2.54   6.0%     54   169.3
shell                           1.40   3.3%    840     6.0
cargo test                      0.78   1.8%    114    24.6
wait: subagent                  0.38   0.9%      2   692.1
wait: release                   0.34   0.8%      8   151.1
cargo build                     0.31   0.7%     82    13.6
edit files                      0.10   0.2%    455     0.8
web                             0.09   0.2%     29    11.3
remote wait                     0.09   0.2%     12    26.2
read files                      0.07   0.2%    676     0.4

phase                          hours  share  steps  mean s
orient                         34.67  82.0%   4260    29.3
build                           3.36   8.0%    699    17.3
gate                            2.24   5.3%    142    56.9
prove                           0.67   1.6%    111    21.8
release                         0.61   1.4%     51    42.9
research                        0.38   0.9%    117    11.7
ship                            0.35   0.8%     76    16.4
log                             0.02   0.0%      8     7.6

slowest single steps
  pass   18    5563s  bg    shell                bg_4 completed: tools/webgl2/check.sh . 12 2>&1 | tail -25
  pass   12    1862s  tool  wait: gate           Waiting for merged-tree gate
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    8    1800s  tool  wait: gate           Waiting for the gate
  pass    9    1800s  tool  wait: gate           Waiting for the gate sequence
  pass   16    1800s  tool  wait: gate           Waiting for gate or harness
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
```

