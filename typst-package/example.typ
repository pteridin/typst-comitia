#import "./lib.typ" : vote-report, vote

#set heading(numbering: "1.1")

#show heading.where(level: 4): it =>[
    #block(it.body)
]
#show heading.where(level: 5): it =>[
    #block(it.body)
]
#show heading.where(level: 6): it =>[
    #block(it.body)
]
#set par(justify: true)

#outline(depth: 3)

= Define an Input

First you must define an input for our vote method. This input must be a list
with each element of the list containing the ranked choices for an individual:

```typst
#let input = (
    ("Alice", "Charlie"),
    ("Bob", "Charlie", "Alice"),
    ("Charlie", "Alice", "Bob"),
    ("Alice", "Charlie", "Bob"),
    ("Bob", "Alice", "Charlie"),
    ("Tim",)
)
```

#let input = (
    ("Alice", "Charlie"),
    ("Bob", "Charlie", "Alice"),
    ("Charlie", "Alice", "Bob"),
    ("Alice", "Charlie", "Bob"),
    ("Bob", "Alice", "Charlie"),
    ("Tim",)
)

In our example the first person prefered Alice over Charlie and the last person
only voted for Tim. Please note that an Element that contains only one element
must end with a comma, as shown in #("Tim",).

= Retrieve raw results (`#vote`) <RAW>

To retrieve the raw results simply call 

```typst
  #vote(input)
```

The results is JSON and is shown here:

#vote(input)

By default vote operates on Plurality (See @Plurality) vote mode and for ties 
all candidates are retained (and/or eliminated). For other methods of solving
votes see @Methods and for other tie breaker modes see @Ties.


= Retrieve a detailed result report (`#vote-report`)

To retrieve a report of the raw results simply call 

```typst
#vote-report(input)
```

This method takes in the same parameters as `#vote`. For reference see @RAW.

The results are shown in the examples (see @Example-Plurality and @Example-SVT).


= Methods <Methods>

== Plurality <Plurality>

Each ballot selects one candidate. The candidate with the highest number of votes wins. The candidate with the fewest votes is eliminated.

== STV <STV>

Single Transferable Vote is used when multiple seats are to be filled. A quota of votes is calculated. Candidates who reach the quota are elected, and any surplus votes they receive are transferred to remaining candidates according to voter preferences. If no one meets the quota, the candidate with the fewest votes is eliminated and their ballots transferred. The process repeats until all seats are filled.

With only one seat to fill, STV works like Instant-Runoff Voting: voters rank candidates, the least-voted candidate is eliminated each round, and their ballots are transferred to the next preference, until one candidate remains and wins (> 50% of votes).

= Tie Breakers <Ties>

== All

If multiple candidates tie for winning or elimination, all tied candidates share the outcome (all win or all are eliminated).

== Random

If multiple candidates tie, a random selection among the tied candidates determines who wins or is eliminated.

== Count

If multiple candidates tie, the total number of ballots that contain this candidate is summed up. This sum 
is used to break the tie.

= Example Reports for Plurality-Voting <Example-Plurality>

== Tie Method: All

Call:

```typst
#vote-report(input)
```

#vote-report(input)
#pagebreak()

== Tie Method: Random

Call:

```typst
#vote-report(input, ties_method: "Random")
```

#vote-report(input, ties_method: "Random")
#pagebreak()

== Tie Method: Count

Call:

```typst
#vote-report(input, ties_method: "Count")
```

#vote-report(input, ties_method: "Count")
#pagebreak()

= Examples Reports for STV-Voting  <Example-SVT>

== Tie Method: All

Call:

```typst
#vote-report(input, method: "STV", ties_method: "All")
```

#vote-report(input, method: "STV", ties_method: "All")
#pagebreak()

== Tie Method: Random

Call:

```typst
#vote-report(input, method: "STV", ties_method: "Random")
```

#vote-report(input, method: "STV", ties_method: "Random")
#pagebreak()

== Tie Method: Count

Call:

```typst
#vote-report(input, method: "STV", ties_method: "Count")
```

#vote-report(input, method: "STV", ties_method: "Count")