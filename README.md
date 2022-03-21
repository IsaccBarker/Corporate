# Corporate, the esoteric programming language for large businesses.

<p align="center">
    <img src="https://i.giphy.com/l41YfsMeWbVPC5XPy.gif">
</p>

Are you tired of having to write **serious** corporate emails? Me neither! With the corperate language,
your pointlessly formal and not at all understandable communications can also function as full fledged
language. Take that HR!

Corporate is an entirely stack based language, for extra frustration (at least from certain programming
backgrounds).

## Building
Build it like you would any Rust program; install the right tool chain, build, and run! If you don't have
Rust already installed, you can visit [rustup.rs](https://rustup.rs/). You don't even need to clone this
repository to build Corporate, simply run the command below to fetch, build, and install:

```bash
cargo install --git https://github.com/IsaccBarker/Corporate.git
```

It's that easy!

## How it works

Corporate is a turing complete language, which means you can calculate anything you could want with it.
verything your heart can desire! It can't yet actually interact with the system itself (besides writting
to stdout), but we're working on that ;)

Corporate scripts should end in the `.email`, `.msg`, or `.eml` file extension. However, corporate scripts
cannot call other corporate scripts.

### Sound like a human

Programming languages typically do not sound like human speach, although some try (see AppleScript), most fail.
Corporate actually makes you sound like a real person.

The corporation that these emails are being sent from is *very stringent* on their formatting. Each paragraph must
be idented with whitespace (tabs or spaces). The sign off must always be indented one tab (or space) in, and your
name and the word above it must be on seperate lines. The seperation of the emails must be three newlines. See the
below example.

```
Subject: Project Delay (Payment Cluster Deployment)
Original Recipient: N/A
Date: 7/28, 2028, 10:03

Hi Ainsley,
    Regretfully, I must bring to attension two bugs that might effect our bottom line, as fixing these would require
a substantial delay. Following up on your meeting, would you be willing to commucate in person once more? Just wanted
to run changing the project deadline up the flagpoll, and I hope we can sort this out and not involve upper management,
but if nessisary I'll ping Claire and Carolina and have them in the loop regarding this chain. Lets consider our clients
first and foremost, but we may have have to table this to reach our bottom line. I'd like to sincerely apologize to our
clients, if this doesn't work out.

    Regretfully,
    Vivian



Subject: Project Delay (Payment Cluster Deployment)
Original Recipient: Ainsley
Date: 7/28, 2028, 10:28

Hey Vivian,
    Hope you're having a wonderful day, and that this email finds you well. Concerning those two points you've brought up
concerning an unexpected that you and your team have encounted. We don't currently have any leway here, as our next deployment
is only four days away. With two more serious production-exclusive bugs, I'm afraid that this won't track well with your
record here. We can keep deploying the legacy code, but that would require us providing very slow service. It's always an option,
but it would be much better if you could have your team crunch, and get us the software we need to keep providing the exelent
service that this company is known for. We may have to incude upper management if this doesn't work out.

    Kind regards,
    Ainsley
```

### Start

Code execution will always start with the first namespace, and the first function inside that namespace.

### Eggs

`<eggs>` is used to signifiy that anything at all can go here (sans a new line). It will simply look for the word it
expects next, and continue from there. That way, it's possible to have full creative expression in your locked down
corporate email.

### Namespaces

A namespace is simply a new email. Take the above chain as an example; we have both the namespacees "Hey" and "Hi". You can
use these namespacees to house functions, and thus functionality. A new namespace is marked with three new lines, `Subject`
with no tab before it, followed by `Original Recipient` and `Date`. After that, is the start of a new namespace/email.

The name of a namespace is taken either from the parentheses after the subject, or from the date ("yesterday", "2 days ago",
"1 week and 2 days ago") are all perfectly valid.

### Functions

Each function is defined by a block of text starting with a tab and ending with a new line. The name of the function is
the number of words (delimiters " ", ",", ":", ";", "--", "(",  and ")") that are present in the first sentense.

You can call a function in another namespace by saying:

```
(<eggs>see the <function name><eggs> interation stated <namespace ID><eggs>)
, namely the <function name><eggs> requirement <eggs> referenced <namespace ID>
. Following up on your <namespace ID>, would you be willing to <function name>?
```

You can reference namespacees up to 10 emails in the future, by the following:

1. x = 1, "meeting"
2. x = 2, "conference"
3. x = 3, "chat"
4. x = 4, "call"
5. x = 5, "note"
6. x = 6, "letter"
7. x = 7, "download"
8. x = 8, "uplink"
9. x = 9, "conversation"
10. x = 10, "consultation"

You can call a function in the same namespace by saying:

```
(<eggs>see the <function name><eggs> iteration stated in this email<eggs>)
, namely the <function name><eggs> requirement <eggs> refered in this email
```

## Types

There are only two types in Corporate, those being the Integer and the String. String are
enclosed within quotes, and integers being just numbers.

When referencing a namespace, put the word `reference` before the namespace identifier.

If you don't wish to use a Integer literal, as it might not sound professional, you can do what you might do in the
Rockstar programming language and modulo 10 the length of words, followed up by a period:

```
don't feel comfortable

4 % 10   4 % 10   10 % 10

440
```

## Things

You may see `<things>`, and this is a reference to a list of constants. It's a comma
seperated list of whatever, ending with an
[ofxord comma](https://www.grammarly.com/blog/what-is-the-oxford-comma-and-why-do-people-care-so-much-about-it/)
and an and (`, and`).

## The stack

You can push things onto the stack with:

```
I'd like to consider <things>
bring to attension <things>
```

You can also pop things from the stack like so:

```
let's drop <number of element>
I don't have the bandwidth to <eggs> <thing>
```

## Arithmetic

Because Corporate is a stack based language, arthithmetic if pretty simple.

The number of elements refers to how many items on the stack do we want to 

```
addition:
add
include
. With

subtraction:
subtract
discontinue
ignore
. Without

multiplication:
multiple
times
emphasize
boost
. Of

division:
divide
controversial between
don't like 
. Over

modulo:
remainder
```

The result will be popped onto the stack, and the elements that are used
will be popped off.

## Printing

You can print to the console by specifying a key word, and then the value at the
top of the stack will be printed. It will not be poped.

```
apologize to
state
tell
say
print
```

# That's about it!

