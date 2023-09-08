# locker

A demonstration of flock().

```
 $ seq 1 10 | xargs -P10 -n 1 ./target/debug/locker sample
[2] locking file "/tmp/locker-sample/lock"...
[2] now has the lock (after 0 ms)
[4] locking file "/tmp/locker-sample/lock"...
[1] locking file "/tmp/locker-sample/lock"...
[7] locking file "/tmp/locker-sample/lock"...
[10] locking file "/tmp/locker-sample/lock"...
[6] locking file "/tmp/locker-sample/lock"...
[9] locking file "/tmp/locker-sample/lock"...
[8] locking file "/tmp/locker-sample/lock"...
[5] locking file "/tmp/locker-sample/lock"...
[3] locking file "/tmp/locker-sample/lock"...
[2] created database with contents: database created by 2 in pid 12905
[2] database -> "/tmp/locker-sample"
[1] now has the lock (after 4999 ms)
[1] got pre-existing contents: database created by 2 in pid 12905
[1] database -> "/tmp/locker-sample"
[4] now has the lock (after 4999 ms)
[4] got pre-existing contents: database created by 2 in pid 12905
[4] database -> "/tmp/locker-sample"
[7] now has the lock (after 4999 ms)
[7] got pre-existing contents: database created by 2 in pid 12905
[7] database -> "/tmp/locker-sample"
[6] now has the lock (after 4999 ms)
[6] got pre-existing contents: database created by 2 in pid 12905
[6] database -> "/tmp/locker-sample"
[10] now has the lock (after 4999 ms)
[10] got pre-existing contents: database created by 2 in pid 12905
[10] database -> "/tmp/locker-sample"
[9] now has the lock (after 4999 ms)
[9] got pre-existing contents: database created by 2 in pid 12905
[9] database -> "/tmp/locker-sample"
[8] now has the lock (after 4999 ms)
[8] got pre-existing contents: database created by 2 in pid 12905
[8] database -> "/tmp/locker-sample"
[5] now has the lock (after 4999 ms)
[5] got pre-existing contents: database created by 2 in pid 12905
[5] database -> "/tmp/locker-sample"
[3] now has the lock (after 5000 ms)
[3] got pre-existing contents: database created by 2 in pid 12905
[3] database -> "/tmp/locker-sample"
```
