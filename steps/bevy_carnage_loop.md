# Where bevy_carnage_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_carnage_loop.tsv`](bevy_carnage_loop.tsv). Updated 2026-11-01 03:13Z.

## What the last passes shipped

From [`bevy_carnage_loop-audit.tsv`](bevy_carnage_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 0 | 2026-11-01T00:31Z | 2 | 0 | 0 | - | 0 | 0 | - |

## The last 12 passes

```text
12 pass(es), 3636 steps, 28.2 h of pass time

category                       hours  share  steps  mean s
wait: gate                      8.78  31.1%     30  1054.1
wait: build                     6.00  21.3%     47   459.7
model                           4.05  14.3%   1651     8.8
wait: other                     3.33  11.8%     41   292.5
wait: tests                     2.80   9.9%     38   265.4
shell                           1.02   3.6%    548     6.7
wait: clip                      0.80   2.8%     14   204.8
cargo test                      0.61   2.2%     89    24.6
wait: release                   0.23   0.8%      3   275.3
cargo build                     0.18   0.6%     41    15.7
remote wait                     0.09   0.3%     12    26.2
edit files                      0.07   0.2%    340     0.7
web                             0.05   0.2%     16    11.6
git                             0.05   0.2%     96     1.8
push gate                       0.04   0.1%     24     5.6

phase                          hours  share  steps  mean s
orient                         28.24 100.0%   3615    28.1

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
15 pass(es), 3741 steps, 32.3 h of pass time

category                       hours  share  steps  mean s
wait: build                     9.97  30.9%     55   652.9
wait: gate                      8.78  27.2%     30  1054.1
model                           4.14  12.8%   1702     8.8
wait: other                     3.33  10.3%     41   292.5
wait: tests                     2.80   8.7%     38   265.4
shell                           1.02   3.2%    557     6.6
wait: clip                      0.80   2.5%     14   204.8
cargo test                      0.61   1.9%     89    24.6
wait: release                   0.23   0.7%      3   275.3
cargo build                     0.18   0.6%     43    15.0
remote wait                     0.09   0.3%     12    26.2
edit files                      0.08   0.2%    347     0.8
web                             0.05   0.2%     16    11.6
git                             0.05   0.1%     97     1.7
push gate                       0.04   0.1%     24     5.6

phase                          hours  share  steps  mean s
orient                         32.32 100.0%   3720    31.3

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

