# Quadtratic Funding Spec

This repository is a proof-of-concept for a customized quadratic funding protocol.

This protocol must meet certain requirements:
* Funding will commence in "rounds" that will begin and end at specific date & time intervals
* Funders will allocate a lump sum of funds to the funding round, to be distributed algorithmically upon its conclusion
* The funds will NOT be hosted in escrow during the funding round (beta release)
* Projects seeking admission to the funding round must be approved before entering the funding round
* Projects can be approved during an active funding round and entered in the round

## Flow

Projects can apply for an upcoming or active funding round anytime.

Their application must meet certain requirements:
* Basic information (project name, etc.)
* Proposal document according to pre-defined standard

Their proposal must be reviewed by the funding organization
* This will be a manual process, where organization members will periodically review and request changes
* 💡 Perhaps notifications can be configured for this?

Once a funding round begins, users on the platform will cast "votes" in the form of SOL, USDC, or any other 
platform-recognized token.
These "votes" will count toward the Quadratic Funding Algorithm.

## Algorithm



## Escrowing (Future)

As this protocol rolls out in a "soft launch", escrowing should not be required to commence a funding round, 
but one day it should be.
Adding escrow functionality in the future should be a simple plug-and-play.

It can even be integrated as part of the core functionality, but only used when the funding round closes and payments
are disbursed.