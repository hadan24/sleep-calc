# Sleep Calculator

#### To-Do
- [ ] add link where readers can learn more about sleep cycles
- [ ] add gif of output

Have you ever thought you'd get a good night's sleep but somehow still wake up exhausted and
sleepy? That may be due to _sleep cycles_! When we sleep, our brains go through specific cycles
to rest and restore itself. Those cycles have phases that last for specific time ranges, and
waking up in the middle of the later phases of a cycle can make you feel just as tired as
not getting any sleep at all!

I deal with this a lot in my struggle to maintain a consistent sleep schedule (life, y'know? :/).
This sleep calculator is a quick and easy way to help ensure I wake up at the right times in my
cycle. I hope it helps you too if you decide to use it!

This is heavily inspired by https://sleepcalculator.com/, which I've been using for a while. 
However, I wanted to make a CLI version so that I can calculate my sleep needs without an internet
connection :)). This also lets me add some more features that I've been wanting.

## Features
- Given a set bedtime, calculates good wake-up times
    - if invoked with no arguments, assumes the invocation time _is_ the set bedtime
- Given a set wake-up time, calculates good bedtimes
- Given both a bedtime _and_ wake-up time, calculates how many cycles can fit within those times and what a good wake-up time is (assuming you sleep at the given bedtime)

## Examples
```
./sleep-calc

./sleep-calc -b [time]
./sleep-calc -bedtime [time]

./sleep-calc -w [time]
./sleep-calc -waketime [time]

./sleep-calc -b [time] -waketime [time]
./sleep-calc -w [time] -bedtime [time]
./sleep-calc -bedtime [time] -w [time]
```
`-m` flag can also be given to any above invocation to activate 24-hour output mode

### Example Accepted Time Formats
```
"3:00 pm"   // arguments with spaces in between typically must be in quotes
3:00pm
"3 pm"
3pm

// these all parse as 24-hour times
15:00
1500
0300    // 3:00 AM written in a 24-hour format
15
```
I am open to accepting more time formats! If you feel your format should be accepted, please
[open an issue](https://github.com/hadan24/sleep-calc/issues/new) with several examples of the
new format in action.

## Questions, Comments, Concerns, Objections?
[Issues](https://github.com/hadan24/sleep-calc/issues/new),
[feature requests](https://github.com/hadan24/sleep-calc/issues/new),
[bug reports](https://github.com/hadan24/sleep-calc/issues/new), and
actually effort-ful [PRs](https://github.com/hadan24/sleep-calc/compare) are greatly appreciated!
I have some issues made for things I plan to add soon, but am open to more ideas!
