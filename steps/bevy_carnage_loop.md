# Where bevy_carnage_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_carnage_loop.tsv`](bevy_carnage_loop.tsv). Updated 2026-10-31 22:29Z.

## The last 12 passes

```text
12 pass(es), 2434 steps, 22.2 h of pass time

category                       hours  share  steps  mean s
wait: gate                      7.67  34.5%     26  1061.3
wait: build                     4.94  22.2%     32   556.2
wait: tests                     2.56  11.5%     25   368.7
model                           2.52  11.3%   1088     8.4
wait: other                     2.47  11.1%     23   385.8
shell                           0.64   2.9%    367     6.3
wait: clip                      0.64   2.9%      9   254.1
cargo test                      0.20   0.9%     49    14.9
wait: release                   0.20   0.9%      2   360.4
cargo build                     0.11   0.5%     26    14.7
edit files                      0.05   0.2%    228     0.8
web                             0.04   0.2%     12    12.8
remote wait                     0.04   0.2%      7    18.5
eval                            0.03   0.2%     21     5.9
git                             0.03   0.2%     74     1.7

phase                          hours  share  steps  mean s
orient                         22.25 100.0%   2418    33.1

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
13 pass(es), 2537 steps, 26.3 h of pass time

category                       hours  share  steps  mean s
wait: build                     8.92  33.9%     40   802.5
wait: gate                      7.67  29.1%     26  1061.3
model                           2.61   9.9%   1137     8.3
wait: tests                     2.56   9.7%     25   368.7
wait: other                     2.47   9.4%     23   385.8
shell                           0.65   2.5%    376     6.2
wait: clip                      0.64   2.4%      9   254.1
cargo test                      0.20   0.8%     49    14.9
wait: release                   0.20   0.8%      2   360.4
cargo build                     0.11   0.4%     28    13.6
edit files                      0.06   0.2%    235     0.9
web                             0.04   0.2%     12    12.8
remote wait                     0.04   0.1%      7    18.5
eval                            0.03   0.1%     21     5.9
git                             0.03   0.1%     75     1.6

phase                          hours  share  steps  mean s
orient                         26.33 100.0%   2521    37.6

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

