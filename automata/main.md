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

# 1 Formal Languages of DFA's and NFA's 

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

TODO Example

## 1.6 Automata with transitions on $\epsilon$
*9-20-22*

Why care about $\epsilon$-NFA?
Suppose $L_1,L_2$ are regular. Then there exists DFA $M_1,M_2$ such that $L(M_1) = L_1$ and $L(M_2) = L_2$.
...
Basically, we can use $\epsilon$-NFA to prove closure properties of regular languages.

## 1.7 Equivalence of $\epsilon$-NFA and NFA

Want to build an NFA $N^\prime$ such that for an $\epsilon$-NFA $N$

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

# 2 Regular Expressions

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

__Pf__

Let $M$ be $\{Q, \Sigma, \delta, q_0, F\}$ be a DFA such that $L(M) = L$.
For each of these closures, we will construct DFA $M^\prime$.

1. $\overline{L(M)}$ is accepted by the same machine. Just let $F^\prime = Q \setminus F$.

2. $L(M_1) \cap L(M_2)$ 
$$\{Q_1 \times Q_2, \Sigma, \delta^\prime, (q_{01}, q_{02}), F^\prime\}$$
- $\delta^\prime((q_1, q_2), a) = (\delta_1(q_1,a), \delta_2(q_2,a))$
- $F^\prime = \{ (q_1,q_2) \in Q_1 \times Q_2 \mid q_1 \in F_1 \land q_2 \in F_2 \}$

3. $L(M_1) \cup L(M_2)$ is the same construction except
- $F^\prime = \{ (q_1,q_2) \in Q_1 \times Q_2 \mid q_1 \in F \vee q_2 \in F_2 \}$

4. See schematic for construction.

5. See schematic for construction.

Also we can prove union is closed a different way.
$$L_1 \cup L_2 = \overline{\overline{L_1 \cup L_2}}
= \overline{\overline{L_1} \cap \overline{L_2}}$$
As complement and intersection perserve regular, regular languages are closed under union.

Here is another example of this process. Is set-minus closed?
$$L_1 - L_2 = L_1 \cap \overline{L_2}, \text{ so set minus is closed.}$$


## 2.2 What is a Regular Expression? 
*10-6-22*
A regular expression is a finite expression for a possibly infinite language. 
For which languages can we express them in regular expressions? Regular languages.

__Def__ $\Sigma$ is an alphabet. Let's talk about *regular expressions over this alphabet*.
The definition of a Regular Expression (rexp) over $\Sigma$ is inductive.

Base:
$$\epsilon \text{ is a regular expression}$$
$$\phi \text{ is a regular expression}$$
$$a \in \Sigma \text{ is a regular expression}$$

If $x, y$ are regular expressions, then so is

\begin{enumerate}
\item[(i)] xy
\item[(ii)] x+y
\end{enumerate}

If $x$ is a regular expression, then so is $x^*$.

__Def__ For rexp $x, L(x)$ is the language described by $x$.

__Examples__ 

$$L(\epsilon) = \{ \epsilon \}$$
$$L(\phi) = \{ \}$$
$$L(a) = \{ a \}$$
$$L(xy) = L(x)L(y)$$
$$L(x+y) = L(x) \cup L(y)$$
$$L(x^*) = (L(x))^*$$

Let $\Sigma = \{ 0,1 \}$. Then
$$L(101) = L(1)L(0)L(1) = \{1\}\cdot\{0\}\cdot\{1\} = \{101\}$$
$$L((0+1)^*) = (L(0+1))^* = (L(0)\cup L(1))^* = ({0,1})^* = \Sigma^*$$

Intuitively, $(0+1)^*$ says "0 or 1 repeated zero or more times."

$$L((0^*1^*)^*) =?$$

First, look at $L(0^*1^*) = \{\epsilon, 0, 1, \ldots \}$. So 
$$L((0^*1^*))^* = \{\epsilon, 0, 1, \ldots \}^* = \Sigma^* = L((0+1)^*)$$

This shows that different looking regular expressions can describe the same 
language.

$$L(00^*) = \text{ 0 followed by 0 repeated 0 or more times.}$$

__Notation__ $xx^* = 0^+$, or $x$ repeated *one or more times*.

__Example__ All strings containing 10 and 01

$$...10...01...$$
$$...01...10...$$
$$.....101.....$$
$$.....010.....$$
\begin{align*}
&  (1+0)^*10(1+0)^* 01(1+0)^* \\
+ & (1+0)^*01(1+0)^*10(1+0)^*		\\
+ & (1+0)^* 101 (1+0)^*		\\
+ & (1+0)^* 010 (1+0)^*		\\
\end{align*}

__Example__ All strings containing exactly two 1's or exactly three 1's.

$$0^*1 0^* 1 0^* + 0^* 1 0^* 1 0^* 1 0^*$$

__Example__ All strings of even length.

$$(00 + 01 + 10 + 11)^*$$

__Example__ All strings with even number of 1's.

$$\ldots1 \ldots 1 \ldots \mid \leftarrow \text{ also in L. } \rightarrow$$
$$0^* + (0^* 1 0^* 1 0^*)^*$$

__Example__ All strings with odd number of 1's.

$$\ldots 1 \ldots 1 \ldots \mid \leftarrow \text{ even number of 1's } \rightarrow$$
$$(0^*10^*)(0^*10^*10^*)^*$$

__Example__ All strings with even number of 1's and even number of 0's.

$$00 \mid \leftarrow \text{ also in L. } \rightarrow $$
$$11 \mid \leftarrow \text{ also in L. } \rightarrow $$
$$01 \mid \leftarrow \text{ must have 01 or 10 } \rightarrow$$
$$10 \mid \leftarrow \text{ must have 01 or 10 } \rightarrow$$

$$\left(00 +	11 + (10 + 01)(00+11)^*(01+10)\right)^* $$

## 2.3 Equivalence of rexp and Regular Languages
*10-11-22*

__Thm__ Let $\Sigma$ be an alphabet. A language $L$ over $\Sigma$ is regular
$\iff$ there is a rexp $\alpha$ over $\Sigma$ such that 
$$ L(\alpha) = L.$$

__Pf__ Suppose $\alpha$ is a rexp over $\Sigma$. WTS that $L(\alpha)$ is regular by producing a FA that accepts $L(\alpha)$.

If $\alpha = \epsilon$, then $L(\alpha) = 	\{ \epsilon \}$, which is clearly regular.

If $\alpha = \phi$, then $L(\alpha) = \phi,$ which is clearly
regular.

If $\alpha = a, a \in \Sigma$, then $L(\alpha) = 	\{ \alpha \}$,which is clearly regular.

Otherwise, start with a simple machine
```
		alpha
	>q---------->f
```


and "refine the machine" based on label on transitions until we have an FA. 

```
								x
	x+y						|--------v
p---------->q		===> 	p------->q
								y
```
```
								
	xy							x		y
p---------->q		===> 	p------->p'------>q
								
```
```
								
	x*							epsilon			epsilon	
p---------->q		===> 	p------------>p'------------>q
										 ()
										  x
```
Do this construction until transitions are on $a \in \Sigma$ or $\epsilon$.

Now we want to show, if we have a regular language, then we can write a regular expression for it.

Suppose $L$ is regular. Then there exists a DFA $M$ such that $L(M) = L$. Want to start with $M$ and derive a regular expression for $L(M)$.

Add a new initial state $i$ and final state $f$ to $M$ and modify appropriately i.e. make $i$ the only initial state and $f$ the only final state, use $\epsilon$-transitions to not modify the language.

Eliminate states and transitions of $M$ to derive 

```
	rexp
>o-------->o
```

What does eliminating transitions mean? If you have

```
	x								x+y
p-------->q				===> 	p-------->q
|		  ^	
+---------+
	y
```

\newpage
Eliminating state $p$ is a bit more complicated.


```
		 c
     b1	 ()  a1 
p_1----->p------>q1
.	   /  \      . 
.	  /    \     .
.	 /      \    .
.	/        \   .
pn_/          \__qm
```

If we want to delete $p$, then 

```
for each i
	for each j
		add pi to qj a transition labeled bi c* aj
```

If you continue this process, eventually you will be left with

```
	   rexp
>i------------>f
```

as desired.

## 2.4 Nonregular Languages?

Set of all languages is $\mathcal{P}(\Sigma^*)$, which is uncountable. But DFA's are enumerable, so there must exists nonregular languages.

- Every finite $L$ is regular.

***
*Sketch of the Pumping Lemma:*

If $L$ is regular, then every "long string" in $L$ satisfies some property $P$.

To show a language $L$ is not regular, argue it has a *specific* "long string" that breaks property $P$.

***

Let's actually derive the Pumping Lemma.

Suppose $L$ is regular.

Let $n$ be the number of states in the smallest DFA accepting $L$.

Let $w \in L$ be such that $|w| \geq n$.
So $w = a_1a_2\ldots a_{n-1}a_n \ldots$.

Think of what the smallest machine does on $w$. Since $w \in L$, there exists a path from $q_0$ to a final state $f$.

```
	a_1		a_2			 an-1	 an	
>q0----->o------>	...	------>o----->o~~~~~~~~>f

|-------------------------------------|
				n+1 states
```

Number of states is $n$, so therefore at least one state repeats in our path. Call this state $q$

```
						w
|-----------------------------------------------------------|
	a_1		a_2			 			 an-1	 an	
>q0----->o------>  ... q ... q ...	------>o----->o~~~~~~~~>f

|-----------------------|----|------------------------------|
		x				  y			      	z

```

$x$ could be $\epsilon$. $z$ could be $\epsilon$. But $|y| \geq 1$ since this is a DFA.

$|xy| \leq n$.

*Importantly*, $xz \in L$, $xyz \in L$, and $xy^2z \in L$. Note,
$$xy^nz \in L, \forall n \in \mathbb{N}_{\geq 0}$$

***
	

## 2.5 The Pumping Lemma

__Lemma (The Pumping Lemma)__ Let $L$ be a regular language, then there exists a constant $n$ that depends only on $L$ such that every string $w \in L$ with $|w| \geq n$ can be written as
$$w=xyz$$
such that
\begin{enumerate}
\item[(i)] $|y| \geq 1$,
\item[(ii)] $|xy| \leq n$,
\item[(iii)] and $xy^kz \in L, \forall k \in \mathbb{N}_{\geq 0}$. 
\end{enumerate}

***

Preview of a nonregular language: { 01, 0011, 000111, \ldots \}.

## 2.6 Examples of the Pumping Lemma
*10-13-22*

__Ex 1__ Consider $\{0^i1^i \mid i \in \mathbb{N} \} = \{01,0011,000111,\ldots\}$.

__Pf__ Let $n$ be the constant of Pumping Lemma. Consider
$$w = 0^n1^n.$$
$w \in L$ and $w \geq n$.
If $L$ is regular, then there exists $x,y,z$ such that
$w =xyz$ and
\begin{enumerate}
\item[(i)]$|y| \geq 1$
\item[(ii)] $|xy| \geq n$
\item[(iii)] $\forall k \geq 0, xy^kz \in L$
\end{enumerate}
Because of (ii) and (iii), anyway we write $w = xyz$, $y$ must be one or more 0's.
Let $y = 0^a$ where $1 \leq a \leq n$. Then 

$$xy^2z = 0^{(n-a)+a+a}1^n$$

This is not in the language because $n + a > n$. Therefore $L$ is not regular.


__Ex 2__ $\{ w \mid w \text{ has equal number of 0's and 1's}\}$.

Pick $010^n1^n$. $w \in L, |w| \geq n$. Consider $w = xyz, x=\epsilon, y=01, z=0^n1^n$.
This string will not work, because we can pump $y$.

However, if we pick the same string as the last one. This argument will work.

__Note__ There exists languages that are not regular that cannot be proved to be nonregular with the Pumping Lemma.

__Ex 3__ Show $L = \{ 1^i \mid i \text{ is a prime}$ is not regular.

__Pf__ Let $n$ be the constant of Pumping Lemma. Let $p$ be a prime such that $p \geq n$.
Let $w = 1^p$. Clearly, $w \in L$ and $|w| \geq n$. 
If $L$ is regular, then there exists $x,y,z$ such that
$w =xyz$ and
\begin{enumerate}
\item[(i)]$|y| \geq 1$
\item[(ii)] $|xy| \geq n$
\item[(iii)] $\forall k \geq 0, xy^kz \in L$
\end{enumerate}
Anyway $w$ is written as $xyz$, $y=1^a$ where $1 \leq a \leq p$. Then

$$|xy^0z| = p-i$$
$$|xy^1z| = p$$
$$|xy^2z| = p+ i$$
$$|xy^3z| = p+ 2i$$

In general,
$$|xy^{j+1}z| = p + j\cdot i$$
Pick value for $j$ such that $p + j\cdot i$ is not prime. How about $j=p$
$$|xy^{p+1}z| = p + pi = p(1 + i) \implies \text{ not a prime, as $1+i \geq 2$. }$$

## 2.6 Grammars

Parser in compiler:

- Checks syntax
- Aware of the grammar for the language

Example

\begin{center}
meal $\to$ ABC 					\\
colA $\to$ salad	| 	roll 	\\
colB $\to$ fish		|	veg	 	\\
colC $\to$ cake		|	coffee	\\
\end{center}

meal, colA, colB, colC are the  *variables* or *nonterminals*. salad,roll,fish,veg,
coke,coffee are the *terminals*.

Above is a grammar with 7 rules (this is because salad | roll counts as two rules).

meal is the *start symbol*. The centered collection are called the 
*rules* or *productions*.

A *derivation* is a string of terminals obtained by replacing each variable with
a terminal according to the rules.

Derivation of a string:

\begin{center}
Start at the start symbol								\\

meal  $\implies$ colA colB colC							\\
		$\implies$ salad colB colC							\\
	  $\implies$ salad fish colC						\\
	  $\implies$ salad fish coffee						\\
\end{center}

This is called a *leftmost derivation*, where we replace the leftmost variable.


__Def__ Formally, a __grammer__ $G = (V,T,P,S)$.

- $V$ is the set of variables or nonterminals
- $T$ is the set of terminals
- $P$ is the set of rules or productions
- $S$ is the start symbol, where $S \in V$.

By convention, we use:

- $x,y,z$ for elements of $V$.
- $a,b,c$ for elements of $T$.
- $u,v,w \in T^*$. For example, salad colB colC $\in (V \cup T)$.
- $\alpha, \beta \in (V \cup T)^*$. For example, salad fish coffee $\in T^*$.

In general, a rule looks like:
$$ \alpha \to \beta, \text{ where $\alpha,\beta \in (V \cup T)^*$} $$

$$\alpha \underset{G}{\overset{*}{\implies}} \beta $$

This notation means there exists a derivation in $G$ in 0 or more steps from $\alpha$ to $\beta$.
That is $\alpha$ can *produce* $\beta$ in 0 or more steps using the rules of grammar
$G$.

The language described by $G$ is the set $L(G)$ where

$$L(G) = \{ u \in T^* \mid S \underset{G}{\overset{*}{\implies}} u\}$$


Typically, we use $S$ for the start symbol. Often the start symbol is list in the first rule of the grammar.

***

__Example__

\begin{align*}
S	&	\to		0S1 \mid 01	\\
\end{align*}

\begin{tabular}{c|c}
$S \implies 01$		&		$S \implies 0S1$    	\\
					&		$\implies	00S11$ 		\\
					&		$\implies	000S111$	\\
					&			$\vdots$			\\
\end{tabular}

So $L(G) = \{ 0^n 1^n \mid n \geq 1 \}$.


## 2.7 Chomsky Hierarchy

What sort of grammars can generate what kind of languages?

\begin{tabular}{c c c}
name of grammar		&		type of grammar		&	restriction on rules \\
\hline

Unrestricted		&	0						&	none				\\
context sensitive	&	1						&	$|\alpha| \leq |\beta|$	\\
context-free		&	2						&	$\alpha \in V$			\\
regular				&	3						&	$\alpha \in V$ and ($\beta \in T \times V$ or $\beta \in T$)
\end{tabular}

$L$ is context-free if there exists a context-free grammar $G$ such that $L(G) = L$.
