# Where bevy_carnage_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_carnage_loop.tsv`](bevy_carnage_loop.tsv). Updated 2026-11-01 09:53Z.

## What the last passes shipped

From [`bevy_carnage_loop-audit.tsv`](bevy_carnage_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 0 | 2026-11-01T00:31Z | 2 | 0 | 0 | - | 0 | 0 | - |
| 15 | 2026-11-01T03:37Z | 10 | 0 | 4 | v0.8.2 | 9 | 1 | - |
| 16 | 2026-11-01T07:10Z | 10 | 0 | 6 | v0.8.3 | 11 | 2 | - |
| 17 | 2026-11-01T09:53Z | 8 | 0 | 4 | v0.8.4 | 9 | 1 | - |

## The last 12 passes

```text
12 pass(es), 4852 steps, 35.0 h of pass time

category                       hours  share  steps  mean s
wait: gate                     10.64  30.4%     35  1094.7
wait: build                     6.89  19.7%     55   451.0
model                           5.66  16.2%   2222     9.2
wait: other                     3.76  10.7%     49   276.0
wait: tests                     3.07   8.8%     47   234.9
wait: clip                      1.37   3.9%     28   175.7
shell                           1.25   3.6%    740     6.1
cargo test                      0.78   2.2%    112    25.0
wait: subagent                  0.38   1.1%      2   692.1
cargo build                     0.29   0.8%     59    17.9
wait: release                   0.26   0.7%      6   156.8
remote wait                     0.09   0.3%     12    26.2
edit files                      0.08   0.2%    438     0.7
web                             0.07   0.2%     24    11.2
push gate                       0.07   0.2%     37     6.4

phase                          hours  share  steps  mean s
orient                         30.17  86.3%   4053    26.8
gate                            1.66   4.7%    106    56.4
build                           1.61   4.6%    409    14.1
prove                           0.67   1.9%    111    21.8
release                         0.49   1.4%     35    50.8
ship                            0.22   0.6%     63    12.7
research                        0.15   0.4%     41    12.8

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
17 pass(es), 4959 steps, 39.1 h of pass time

category                       hours  share  steps  mean s
wait: build                    10.86  27.8%     63   620.8
wait: gate                     10.64  27.3%     35  1094.7
model                           5.75  14.7%   2275     9.1
wait: other                     3.76   9.6%     49   276.0
wait: tests                     3.07   7.9%     47   234.9
wait: clip                      1.37   3.5%     28   175.7
shell                           1.25   3.2%    749     6.0
cargo test                      0.78   2.0%    112    25.0
wait: subagent                  0.38   1.0%      2   692.1
cargo build                     0.29   0.7%     61    17.3
wait: release                   0.26   0.7%      6   156.8
edit files                      0.09   0.2%    445     0.8
remote wait                     0.09   0.2%     12    26.2
web                             0.07   0.2%     24    11.2
push gate                       0.07   0.2%     37     6.4

phase                          hours  share  steps  mean s
orient                         34.25  87.7%   4160    29.6
gate                            1.66   4.3%    106    56.4
build                           1.61   4.1%    409    14.1
prove                           0.67   1.7%    111    21.8
release                         0.49   1.3%     35    50.8
ship                            0.22   0.6%     63    12.7
research                        0.15   0.4%     41    12.8

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

