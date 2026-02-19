# Sleep Calculator

Have you ever thought you'd get a good full night's sleep but somehow still wake up exhausted
and sleepy? That may be due to _sleep cycles_! When we sleep, our brains go through specific
cycles to rest and restore itself. Those cycles have phases that last for specific time ranges,
and waking up in the middle of the later phases can make you feel just as tired as not getting
any sleep at all! (_note: maybe add a link to learn more here_)

I deal with this a lot in my struggle to maintain a consistent sleep schedule (life, y'know? :/).
This sleep calculator will be a quick and easy way for me to ensure I wake up at the right times
in my cycle. I hope it helps you too if you decide to use it!

This is blatantly inspired by https://sleepcalculator.com/, which I've been using. However, I
wanted to make a CLI version so that I can calculate my sleep needs without an internet connection
:)). This also lets me add some more features that I've been wanting.

## Planned Features
- the original 2 (calculate bedtimes from wake-up time, calculate wake-up times from current bedtime)
- find wake-up times from _chosen_ bedtime
- finding all the above for power naps (_note: research power nap lengths_)
- given a proposed bedtime and wake-up time, calculate how many cycles can fit in

## Resources For Myself
- https://docs.rs/time/latest/time/
- https://rust-cli.github.io/book/tutorial/cli-args.html
- https://docs.rs/clap/latest/clap/