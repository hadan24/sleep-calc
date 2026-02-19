- [x] [given bedtime of _now_, calc wake-up times (default)](#given-bedtime-of-now-calc-wake-up-times-(default))
- [x] [given chosen wake-up time, calc bed times](#given-chosen-wake-up-time-calc-bed-times)
- [x] [given _chosen_ bed time, calc wake-up times](#given-chosen-bed-time-calc-wake-up-times)
- [ ] finding all the above for power naps (_note: research power nap lengths_)
- [x] [given a proposed bedtime and wake-up time, calculate how many cycles can fit in](#given-a-proposed-bedtime-and-wake-up-time-calculate-how-many-cycles-can-fit-in)

---
---

### given bedtime of _now_, calc wake-up times (default)
```
> sleep-calc    // assuming it is 11:45 PM at call time

# cycles    wake-up times
6           9:00 AM (recommended!)
5           7:30 AM (recommended!)
4           6:00 AM
3           4:30 AM
2           3:00 AM
1           1:30 AM
```


### given chosen wake-up time, calc bed times
```
> sleep-calc -w 8:00am
> sleep-calc -wake 8:00am
> sleep-calc -wakeup 8:00am

# cycles    bed times
6           10:45 PM    (recommended!)
5           12:15 AM    (recommended!)
4           1:45 AM
3           3:15 AM
2           4:45 AM
1           6:15 AM
```

### given _chosen_ bed time, calc wake-up times
```
> sleep-calc 10:00pm
> sleep-calc -b 10:00pm
> sleep-calc -bed 10:00pm
> sleep-calc -bedtime 10:00pm

# cycles    wake-up times
6           7:15 AM (recommended!)
5           5:45 AM (recommended!)
4           4:15 AM
3           2:45 AM
2           1:15 AM
1           11:45 PM
```

### the above, but for power naps
```
> sleep-calc -n     // assuming it is 2:00 PM at call time
> sleep-calc -nap   // assuming it is 2:00 PM at call time
> sleep-calc -b 2:00pm -n

If you start your nap at 2:00 PM, waking up at X:XX PM is recommended for an effective power nap.

> sleep-calc -w 2:00pm -n

If you need to wake up at 2:00 PM, starting your nap at X:XX PM is recommended for an effective power nap.
```

### given a proposed bedtime and wake-up time, calculate how many cycles can fit in
```
> sleep-calc -b 10:00pm -w 5:00am

If you sleep at 10:00 PM, you can get 4 cycles of sleep in before 5:00 AM.
Waking up at 4:15 AM is recommended to avoid interrupting the next sleep cycle.
If the delay is acceptable, wake up at 5:45 AM to get 1 more cycle of sleep.
```

