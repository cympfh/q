# q

## What is q?

```
$ q -- echo 1
$ q
1

$ q -- echo 2
$ q -- echo 3
$ q -- echo 4
$ q -n 2
2
3

$ q -n 2
4
No more q!!
```

## Usage

### push a job

```bash
$ q -- <command>
```

`<command>` is shell command what you want to push/to do later.
`--` is a delimitar. This is optional but recommended for non-ambiguious command parsing.

`<command>` に push (つまり後で実行) したいコマンドを書く.
ただしコマンドの曖昧性を排除するため `--` を前につけておくことを推奨.

### pop jobs (and execute)

```bash
$ q
```

`q` pops a job, then execute it.

単に `q` と実行すればキューから一つジョブを取り出して実行する.

`q` can pop multiple jobs and execute them.

複数取り出して逐次実行する為に次のオプションを使う.

```bash
$ q [-n/--num <num>] [-i/--interval <sec>]
```

`-n` is the number of jobs you want to pop.
`-i` is the interval seconds for jobs.

`-n` で取り出すジョブの個数を指定する.
`--interval` はその際にジョブとジョブの間に何秒間のインターバルを置くかを指定する.

### (subcommand) show

```bash
$ q show
```

過去キューに登録されたジョブを全て表示する.

### (subcommand) revive

```bash
$ q revive <JOB_ID>
```

一度失敗したジョブをもう一度キューに入れ直す.

### (subcommand) rm

```bash
$ q rm <JOB_ID>
```

リストから完全に消す

## How to clear

すべてのリスト履歴を抹消する方法

```bash
$ rm .q
```

