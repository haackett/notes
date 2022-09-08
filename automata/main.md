# Automata Theory

# 0 Motivation

*8-23-22*
*8-25-22*

There are many more problems in computing than solutions (programs).

__Pf__ \
There exists a bijection between the set of programs and the naturals.\
Decision Problems can be represented by 
$${\pi : \pi : \mathbb{N} \rightarrow \{0,1\}}.$$
There exists a bijection between the above set and $\mathcal{P}(\mathbb{N})$. Therefore, 

$$|Programs| < |Decision Problems|.$$

And since there are less decision problems than problems in general, we have 

$$|Programs| << |Problems|.$$

# 1 Formal Languages

## 1.1 Motivation
*8-30-22*

\begin{center}
	Decision problem $\longleftrightarrow$ Formal Language
\end{center}

\begin{center}
Problem: PRIME\\
Is $x$ prime? $\longleftrightarrow$ Is $x \in L = \{10,11,101,111,\dots \}$
\end{center}

Consider the problem $n$-PRIMES, where the goal is to return the number of primes below some $x$. If there is an "easy" solution to this problem then there exists an easy solution to PRIME. 

Model		     	Language			Grammer			Applications
------------------ -------------------- --------------- -----------------------
Finite Automata		Regular				Regular			Lexical analysis, RegEx
Push-down Automata  Context-free		Context-free	Parsing (compilers)
Turing Machines		"Solvable"			Unrestricted	Computability Theory

## 1.2 Basics/Notation

__DEF__ an alphabet $\Sigma$ is a finite, non-empty, set of symbols.

__DEF__ a word/string $w$ is a sequence of symbols $w_1w_2...w_n$ where $w_i\in \Sigma$

__NOTATION__ we write the empty word $\epsilon$.

__DEF__ the length of a word $w$, written $|w|$, is the number of symbols in $w$.

__NOTATION__ $\Sigma^k =$ the set of words over $\Sigma$ of length exactly $k$.

__NOTATION__  $\Sigma^*$ denotes all possible strings over $\Sigma$. Notice this means

$$\Sigma^* = \bigcup_{i=0}^n \Sigma^i.$$

__NOTATION__ $\Sigma^+ = \Sigma^* \setminus \Sigma^0$. 

__DEF__ Concatination of string $y$ with $x$, denoted $xy$ is

$$xy = x_1...x_ny_1...y_n.$$

Notice $x\epsilon = \epsilon x = x$

__DEF__ A language $L$ over an alphabet $\Sigma$ is a subset of $\Sigma^*$.

__NOTATION__ $\phi$ denotes the empty language where $\phi = \{\}$. Notice $\phi \neq \{\epsilon\}$.

## 1.3 Deterministic Finite Automata
*9/1/22*

__Deterministic__ refers to that for each state, for each symbol, there is exactly one transition.

Each state "remembers" something. If a string takes you from the initial state to some other state $q$, it means somethings.

A __dead state__ is one from which no matter what symbols are input after, the machine will never reach an accept state.

__Def__ Formally, a DFA is a set $\{Q, \Sigma, \delta, q_0, F\}$ where

-	$Q$ is a finite set of states $\{q_0, q_1, \dots, q_n \}$.
- $\Sigma$ is a finite alphabet on which the machine works.
- $\delta : Q \times \Sigma \rightarrow Q$ describes the transitions.
- $q_0$ is the initial state.
- $F$ is a set of final states $\{f_1, \dots, f_m \}$.

__Def__ The **extended transition function** is $\hat{\delta}$. Define by induction on the length of string $w$ for some state $q$.

$$\hat{\delta}(q, \epsilon) = q$$

Now, consider some larger string $w$. There is at least one symbol in $w$. So

$$w = xa \text{ where a is the last symbol in $w$ }$$
Then
$$\hat{\delta}(q, w) = \delta(\hat{\delta}(q,x),a).$$

__Def__ A machine accepts a string $w \iff \hat{\delta}(q_, w) \in F$.

__Def__ For a machine $A$, $L(A)$ denotes the language the machine accepts, which is 

$$\{ w \in \Sigma^* \mid \hat{\delta}(q_0, w) \in F \}. $$

__Def__ a language $L$ is regular if there exists a DFA $A$ such that $L(A) = L$.

## 1.4 Nondeterministic Finite Automata
*9-8-22*

Informally, given input $w$, $w$ is accepted by a NFA $N$ if starting from start state there
is a way to read all of input $w$ and reach a final state. Otherwise, $w$ is not accepted.

__Def__ Formally, a NFA $N$ is a set $\{Q,\Sigma, \delta, q_0, F \}$ where

- $Q$ is a finite set of states $\{q_1, \dots, q_n \}$
- $\Sigma$ is a finite alphabet
- $\delta : Q \times \Sigma \rightarrow \mathcal{P}(Q)$.
- $q_0$ is the initial state.
- $F$ is a set of final states $\{f_1, \dots, f_m \}$.
