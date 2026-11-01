# Where bevy_carnage_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_carnage_loop.tsv`](bevy_carnage_loop.tsv). Updated 2026-11-01 07:10Z.

## What the last passes shipped

From [`bevy_carnage_loop-audit.tsv`](bevy_carnage_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 0 | 2026-11-01T00:31Z | 2 | 0 | 0 | - | 0 | 0 | - |
| 15 | 2026-11-01T03:37Z | 10 | 0 | 4 | v0.8.2 | 9 | 1 | - |
| 16 | 2026-11-01T07:10Z | 10 | 0 | 6 | v0.8.3 | 11 | 2 | - |

## The last 12 passes

```text
12 pass(es), 4285 steps, 32.3 h of pass time

category                       hours  share  steps  mean s
wait: gate                     10.21  31.7%     34  1081.5
wait: build                     6.29  19.5%     49   461.8
model                           4.90  15.2%   1962     9.0
wait: other                     3.57  11.1%     44   292.1
wait: tests                     3.00   9.3%     44   245.1
shell                           1.15   3.6%    653     6.3
wait: clip                      1.09   3.4%     22   178.2
cargo test                      0.66   2.0%     97    24.5
wait: subagent                  0.38   1.2%      2   692.1
wait: release                   0.25   0.8%      5   178.1
cargo build                     0.21   0.7%     46    16.6
remote wait                     0.09   0.3%     12    26.2
edit files                      0.07   0.2%    397     0.7
web                             0.06   0.2%     20    11.6
push gate                       0.06   0.2%     32     6.6

phase                          hours  share  steps  mean s
orient                         29.37  91.0%   3868    27.3
gate                            1.20   3.7%     94    46.0
build                           1.04   3.2%    207    18.1
release                         0.49   1.5%     35    50.8
research                        0.14   0.4%     39    13.2
ship                            0.01   0.0%     15     2.5

slowest single steps
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
  pass    8    1800s  tool  wait: gate           Waiting for the gate
```

## Every pass recorded

```text
16 pass(es), 4391 steps, 36.3 h of pass time

category                       hours  share  steps  mean s
wait: build                    10.26  28.2%     57   647.9
wait: gate                     10.21  28.1%     34  1081.5
model                           4.99  13.7%   2014     8.9
wait: other                     3.57   9.8%     44   292.1
wait: tests                     3.00   8.2%     44   245.1
shell                           1.15   3.2%    662     6.3
wait: clip                      1.09   3.0%     22   178.2
cargo test                      0.66   1.8%     97    24.5
wait: subagent                  0.38   1.1%      2   692.1
wait: release                   0.25   0.7%      5   178.1
cargo build                     0.21   0.6%     48    15.9
remote wait                     0.09   0.2%     12    26.2
edit files                      0.08   0.2%    404     0.7
web                             0.06   0.2%     20    11.6
push gate                       0.06   0.2%     32     6.6

phase                          hours  share  steps  mean s
orient                         33.45  92.1%   3974    30.3
gate                            1.20   3.3%     94    46.0
build                           1.04   2.9%    207    18.1
release                         0.49   1.4%     35    50.8
research                        0.14   0.4%     39    13.2
ship                            0.01   0.0%     15     2.5

slowest single steps
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
  pass    7    1800s  tool  wait: build          Waiting for release build
```

