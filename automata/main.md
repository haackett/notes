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

# Introduction to Regular Languages

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

*9-13-22*

__Def__ The **extended transition function** is $\hat{\delta}$. Define by induction on the length of the string $w$ for some state $q$.

$$\hat{\delta}(q, \epsilon) = \{ q \}$$

Consider non-empty string $w$. If it is not empty then it has a last symbol $w = xa$ where $a$ is the last symbol. Let $\hat{\delta}(q, x) = \{ p_i, \dots, p_k \}$ where $p_i$ is a state. Then 

$$\hat{\delta}(q,w) = \bigcup_{i=0}^k \delta(p_i, a) = \{r_1, \dots, r_m \}.$$

__Def__ A NFA accepts an input string $w$ if and only if $\hat{\delta}(q_0, w) \cap F \neq \emptyset$.

__Def__ Let $A$ be an NFA. Then the language of $A$ is

$$L(A) = \{ w \in \Sigma^* \mid \hat{\delta}(q_0, w) \cap F \neq \emptyset \}.$$

For any language $L$, if there is a DFA that accepts $L$, then there is an NFA
that accepts $L$. Suppose NFA $A$ accepts $L$. Can we build a DFA that also 
accepts $L$. The answer is yes.

Assuming that, a language $L$ is *regular* if there exists a DFA or an NFA that
accepts it. It just so happens that NFA are often easier to write, so this is helpful.

## 1.5 Equivalence of NFA and DFA

*9-15-22*

Would like to show $L(N) = L(M)$. Need to 

1. a construction from $M$ for $N$
2. argue $L(N)=L(M)$.

If I want to simulate $N$ like a DFA, what do I track?
Enough to remember where all $N$ could be when started from its
initial state and given string $w$.

__Pf__


1. Formally, given an NFA $N = \{Q_N, \Sigma_N, \delta_N, q_{0N}, F_N \}$ we want to build DFA $M = \{Q_M, \Sigma_M, \delta_M, q_{0M}, F_M \}$.

- $Q_M = \mathcal{P}(Q_N)$.
- $\Sigma = \Sigma_N = \Sigma_M$
- $q_{0M} = \{q_{0N}\}$.
- $F_M = \{ S \in Q_M \mid S \cap F_N \neq \emptyset \}$.
- For state $S \in Q_M, a \in \Sigma,$

$$\delta_M(S, a) = \bigcup_{p \in S} \delta_N(p,a)$$

2. Want to show $L(N) = L(M)$. Going to show $L(N) \subseteq L(M)$ and $L(M)\subseteq L(N)$. To prove these, prove the following by induction on the length of string $w$.
(a) If $N$ can go from state $q_0$ to state $p$ on reading some string $w$, then $M$ goes from $\{q_{0N} \}$ to a state $S$ where $p \in S$. 
(b) If $M$ does the following, $\{q_0 \} \mapsto S$ then the NFA $N$ $q_0 \mapsto p$ $\forall p \in S$.

## 1.6 Automata with transitions on $\epsilon$
*9-20-22*

Why care about $\epsilon$-NFA?
Suppose $L_1,L_2$ are regular. Then there exists DFA $M_1,M_2$ such that $L(M_1) = L_1$ and $L(M_2) = L_2$.
...
Basically, we can use $\epsilon$-NFA to prove closure properties of regular languages.

## 1.7 Equivalence of $\epsilon$-NFA and NFA

Want to build an NFA N' such that for an $\epsilon$-NFA $N$

$$L(N)=L(N^\prime)$$

If $N$ can go from state $p$ to $q$ by reading symbol $a$, then so should $N^\prime$

More generally, $N$ can go from state $p$ to state $q$ on symbol $a$ as be just 
traversing epsilon transitions. 

\begin{center}
	
E-CLOSURE(p) = $\{ q \in Q \mid q$ can be reach via just epsilon transitions $\}$
\end{center}

For a state $S$, ECLOSE$(S)$ is a set of states, including S, that can be reached
from $S$ only using arcs (transitions) labeled $epsilon$.

- $N^\prime$ has the same set of final states as $N$.
- $N^\prime$ has the same set of states as $N$.
- $N^\prime$ has the same state as $N$.

To construct $N^\prime$:

- Compute ECLOSE$(s), \forall s \in Q_N$.
- For a state $p$, to determine transitions on symbol $a$:

1. Let $R = \bigcup_{e \in ECLOSE(p)} \delta(e, a)$. (The set of states reachable
from a state in ECLOSE$(p)$ by an arc labeled $a$).
2. $R^\prime = \bigcup_{r \in R} ECLOSE(r)$.
3. For each $q \in R^\prime$, add a transition in $N^\prime$ from state $p$ on
symbol $a$.

TODO Example


This construction works.

To show language $L$ is regular we can now do one of the following:
1. Show DFA
2. Show NFA
3. Show $\epsilon$-NFA

# 2 Properties of Regular Languages

## 2.1 Closure Properties of Regular Languages

Let $L_1, L_2$ be languages over some alphabet $\Sigma$.

Some operations:

1. Complement	
$$\bar{L}_1 = \{ w \in \Sigma^* \mid w \notin L_1 \}$$

2. Union
$$L_1 \cup L_2 = \{ w \in \Sigma^* \mid (w \in L_1) \vee (w \in L_2)$$

3. Intersection
$$L_1 \cap L_2 = \{ w \in \Sigma^* \mid (w \in L_1) \land (w \in L_2)$$

4. Concatenation
$$L_1L_2 \text{ or } L_1\cdot L_2 = \{wx \mid w \in L_1, x \in L_2 \}$$

*Ex.* Suppose $L_1 = \{0,01,11,101\}, L_2 = \{0,10\}$. Then
$$L_1L_2 = \{00,010,0110,110,1110,1010,10110\}.$$

5. Kleene-Closure or Kleene-Star

Strings in $L_1^*$ are precisely those gotten as follows:
1. Take as many copies (including zero copies) of as many strings in $L_1$ as 
you want and concatinate in any way can.

$$L_1^* = \{ w_1w_2\ldots w_k \mid k \geq 0 \land w_i \in L_1 \}$$

Notice that $\epsilon \in L_1^*$, always.

*Ex.* $L_1 = \{0,10 \}.$
$$L_1^* = \{ \epsilon, 0, 00, 10, 000, 010, 100, \dots \}$$

Suppose $L_2 = \{00,01,10,11 \}$
$$L_2^* = \{ w \in \Sigma^* \mid |w| = 2n, n\in Z_{\geq 0} \}$$

2. Another way to think of $L^*$:
$$\{\epsilon\} \cup L \cup LL \cup LLL \cup \dots$$

*Preview: regular languages are closed under all of these operations.*
