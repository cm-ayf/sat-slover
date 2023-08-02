# SAT Solver

2023年度Sセメスター 情報数理科学IIIの課題提出用です．

## 使い方

1. https://www.cs.ubc.ca/~hoos/SATLIB/benchm.html からベンチマークをダウンロードする．
1. `.cnf`ファイルが入っているフォルダをレポジトリにコピーする．
   - `/uf*`および`/uuf`は`.gitignore`してある．
1. `cargo run --release uf20-91 satisfiable`などとして実行する．

```
Usage: sat-slover <DIR> <EXPECT>

Arguments:
  <DIR>     
  <EXPECT>  [possible values: satisfiable, unsatisfiable]

Options:
  -h, --help  Print help
```

## ベンチマーク

| set        | time per instance (s) |
| ---------- | --------------------- |
| uf20-91    | 0.000331              |
| uf50-218   | 0.006199              |
| uuf50-218  | 0.013517              |
| uf75-325   | 0.048317              |
| uuf75-325  | 0.104326              |
| uf100-430  | 0.288156              |
| uuf100-430 | 0.687535              |
