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

### push jobs

```bash
$ q -- <command>
```

`--` is a delimitar. Optional but recommended for non-ambiguious command parsing.
`<command>` is shell command what you want to push/to do later.

`<command>` に push (つまり後で実行) したいコマンドを書く.
ただしコマンドの曖昧性を排除するため `--` を前につけておくこと.

### pop jobs

```bash
$ q
```

`q` pops a job, then execute it.

`q` を実行すればキューから一つジョブを取り出して実行する.

```bash
$ q [-n/--num <num>] [-i/--interval <sec>]

`-n` is the number of jobs you want to pop.
`-i` is the interval seconds for jobs.
<!-- `q` is executing `n` jobs. If any job failed, `q` stops immediately. -->

`-n` で取り出すジョブの個数を指定する.
`--interval` はその際にジョブとジョブの間に何秒間のインターバルを置くかを指定する.
<!-- `q` は取り出したジョブを順番に実行するが, 一つでもジョブに失敗した場合は直ちに残り全てを中止する. -->

### (subcommand) show

```bash
$ q show
```

### (subcommand) revive

```bash
$ q revive <JOB_ID>
```
