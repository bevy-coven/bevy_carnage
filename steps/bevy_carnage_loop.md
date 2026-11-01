# Where bevy_carnage_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_carnage_loop.tsv`](bevy_carnage_loop.tsv). Updated 2026-11-01 03:37Z.

## What the last passes shipped

From [`bevy_carnage_loop-audit.tsv`](bevy_carnage_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 0 | 2026-11-01T00:31Z | 2 | 0 | 0 | - | 0 | 0 | - |
| 15 | 2026-11-01T03:37Z | 10 | 0 | 4 | v0.8.2 | 9 | 1 | - |

## The last 12 passes

```text
12 pass(es), 3670 steps, 28.7 h of pass time

category                       hours  share  steps  mean s
wait: gate                      9.19  32.0%     31  1066.8
wait: build                     6.00  20.9%     47   459.7
model                           4.08  14.2%   1668     8.8
wait: other                     3.33  11.6%     41   292.5
wait: tests                     2.82   9.8%     39   259.9
shell                           1.02   3.6%    549     6.7
wait: clip                      0.80   2.8%     14   204.8
cargo test                      0.61   2.1%     90    24.3
wait: release                   0.25   0.9%      5   178.1
cargo build                     0.18   0.6%     41    15.7
remote wait                     0.09   0.3%     12    26.2
edit files                      0.07   0.2%    341     0.7
web                             0.05   0.2%     16    11.6
git                             0.05   0.2%     99     1.8
push gate                       0.04   0.1%     28     5.1

phase                          hours  share  steps  mean s
orient                         28.71 100.0%   3649    28.3

slowest single steps
  pass   12    1862s  tool  wait: gate           Waiting for merged-tree gate
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    8    1800s  tool  wait: gate           Waiting for the gate
  pass    9    1800s  tool  wait: gate           Waiting for the gate sequence
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
  pass    8    1800s  tool  wait: gate           Waiting for the gate
```

## Every pass recorded

```text
15 pass(es), 3775 steps, 32.8 h of pass time

category                       hours  share  steps  mean s
wait: build                     9.97  30.4%     55   652.9
wait: gate                      9.19  28.0%     31  1066.8
model                           4.17  12.7%   1719     8.7
wait: other                     3.33  10.2%     41   292.5
wait: tests                     2.82   8.6%     39   259.9
shell                           1.02   3.1%    558     6.6
wait: clip                      0.80   2.4%     14   204.8
cargo test                      0.61   1.9%     90    24.3
wait: release                   0.25   0.8%      5   178.1
cargo build                     0.18   0.5%     43    15.0
remote wait                     0.09   0.3%     12    26.2
edit files                      0.08   0.2%    348     0.8
web                             0.05   0.2%     16    11.6
git                             0.05   0.2%    100     1.8
push gate                       0.04   0.1%     28     5.1

phase                          hours  share  steps  mean s
orient                         32.79 100.0%   3754    31.4

slowest single steps
  pass   12    1862s  tool  wait: gate           Waiting for merged-tree gate
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    8    1800s  tool  wait: gate           Waiting for the gate
  pass    9    1800s  tool  wait: gate           Waiting for the gate sequence
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    1    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
  pass    7    1800s  tool  wait: build          Waiting for release build
```

