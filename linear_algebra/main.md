# Linear Algebra II 
# Factorization of Linear Maps

# 0 Motivation

*8/23/22*

## 0.1 The Big Picture

Math:

$$hard \longrightarrow simple$$
$$hard \longrightarrow linear$$


## 0.2

\begin{equation}
	\left\{ \begin{aligned} 
		a_{11}x_{1} + a_{12}x_{2} + a_{13}x_{3} = b_{1} \\
		a_{21}x_{1} + a_{22}x_{2} + a_{23}x_{3} = b_{2} \\
		a_{31}x_{1} + a_{32}x_{2} + a_{33}x_{3} = b_{3} \\
	\end{aligned} \right.
\end{equation}



Suppose, $A=PDP^{-1}$, where $D$ is diagonal.

$$A\vec{x}=(A=PDP^{-1})\vec{x} = \vec{b}.$$
$$DP^{-1}\vec{x} = P^{-1}\vec{b.}$$

Let $$\vec{c} := P^{-1}\vec{b}.$$

$$DP^{-1}\vec{x} = \vec{c}.$$

... we've made the "difficult" simple.



## 0.3 Discrete Time Evolution Example

Suppose $\vec{x}_t \in \mathbb{R}^m$ is the state of a system at time A, where $t\in\mathbb{Z}_{\geq 0}$ and the system evolves by:

$$ \vec{x}_{t+1} = A\vec{x}_t$$

where $A : \mathbb{R}^n \rightarrow \mathbb{R}^n$, A linear.

Then

$$\vec{x}_1 = A\vec{x}_0.$$
$$\vdots$$
$$\vec{x}_{n+1} = A\vec{x}_n = A^{n+1}\vec{x}_0.$$

Suppose $A=PDP^{-1}$. Then

$$ A^2 = (PDP^{-1})^2 = (PDP^{-1})(PDP^{-1}) = PD^2P^{-1}$$
$$\implies A^{n+1} = PD^{n+1}P^{-1}.$$

Let $\vec{c} := P^{-1}\vec{x}$. Then

$$PD^{n+1}\vec{c} = \vec{x}_{n+1}$$

## 0.4 Factoring Maps

make a diagram please

## 0.5 Some Goals for Factoring Maps

For a linear map

$$A : V \rightarrow W$$

here are the potential factorizations of $A$ in order of "niceness":

1. Spectral Theorem
2. Diagonalization
3. Jordan Canonical Form
4. Singular Value Decomposition

A string __contains__ a string $b$ i

# 1 Vector Spaces

*8/25/22*


## 1.1 Note

## 1.2 Definition

__Def__ A vector space over a field $F$ is a set $V$ with an addition operation
and a scalar multiplication operation such that $\forall \vec{x},\vec{y} \in V,
a,b\in F:$

1. $\vec{x} + \vec{y} = \vec{y} + \vec{x}$
2. $(\vec{x} + \vec{y}) + \vec{z} = \vec{x} + (\vec{y} + \vec{z})$
3. $\exists \vec{0} \in V : \vec{0} + \vec{x} = \vec{x}$
4. $\exists \vec{-x} \in V : \vec{x} + \vec{-x} = \vec{0}$
5. $a(\vec{x} + \vec{y}) = a\vec{x} + a\vec{y}$
6. $(a+b)\vec{x} = a\vec{x} + b\vec{x}$
7. $1 \cdot \vec{x} = \vec{x}$
8. $a(b\vec{x}) = (ab)\vec{x}$

## 1.3 God-given Examples

1. $\mathbb{R}^n, \forall n \in \mathbb{N}.$ 
2. $\mathbb{C}^n, \forall n \in \mathbb{N}.$ 
3. $M_{nm}(\mathbb{R})$
4. Let $F$ be a field and $S$ a set. Then define $F^S$ as the set of all functions $f: S \rightarrow F$. $F^S$ forms a vector space.

## 1.4 Theorem

__Thm__ Let $V$ be a $F$-v.s.

1. $\vec{0} \in V$ in unique.
2. $\forall \vec{x} \in V$, $\vec{-x}$ is unique.
3. $0 \times \vec{v} = \vec{0}$, $\forall \vec{v} \in V$.
4. $a \times \vec{0} = \vec{0}, \forall a \in F$. 
5. $-1 \times \vec{v} = \vec{-v}, \forall \vec{v} \in V$.
6. $-(\vec{-v}) = \vec{v}, \forall \vec{v} \in V$.

__Pf__ By homework.

## 1.5 Definition

__Def__ $V$ a $F$-v.s. If a subset of $V, U,$ is an $F$-v.s. with respect to the same operations as $V$, then $U$ is a vector subspace. 

## 1.6 Theorem

__Thm__ $V$ an $F$-v.s. If $U$ is a subset, then $U$ is a subspace $\iff$

1. $U \neq \emptyset$
2. $\vec{u},\vec{v}\in U \implies \vec{u} + \vec{v} \in U$.
3. $\vec{u} \in U, a \in F \implies a\vec{u} \in U$.

__Pf__ By homework.

## 1.7 Examples

1. $\mathbb{C}[x] = \left\{ \sum_{i=0}^n a_ix^i : a_i \in \mathbb{C}, n \in Z_{\geq 0}\right\}$.
2. The set of solutions to a linear, constant coefficient, homogeneous ordinary differential equation.
3. $\mathcal{C}^o(\mathbb{R}) = \left\{ f: \mathbb{R} \rightarrow \mathbb{R}, f continuous \right\}$.
4. For any $F$-v.s. $V$, ${\vec{0}}$ and $V$ are subspaces.

## 1.8 Definition
*8/30/22*

__Def__ Let $S = {\vec{v}_1, ..., \vec{v}_n}$ and V an $F$-v.s:


a. A linear combination of the elements of $F$ is a vector of the form
$$a_1\vec{v}_1 + ... + a_n\vec{v}_n, a_i \in F.$$

b. $Span(S) = \left\{ \sum_{i=1}^n a_i\vec{v}_i | a_i \in F \right\}$

c. If some set $W=Span(S)$, then we say $S$ spans $W$. 

## 1.9 Theorem

__Thm__ $V$ an $F$-v.s., let $S=\{\vec{v}_1,...,\vec{v}_n\}$ be a non-empty set,
then the span of $F$ is a vector subspace.

__Pf__ 
i. $$0\vec{v}_1 + ... + 0\vec{v}_n \in Span(S).$$
$$ 0\vec{v}_1 + ... + 0\vec{v}_n = \vec{0}.$$
So $Span(S) \neq \emptyset$.

ii. Let $\vec{x},\vec{y} \in Span(S)$.

$$\vec{x} = \sum_{i=1}^n a_i\vec{v}_i.$$
$$\vec{y} = \sum_{i=1}^n b_i\vec{v}_i.$$
$$\vec{x} + \vec{y} = \sum_{i=1}^n (a_i + b_i)\vec{v}_i .$$
So $\vec{x} + \vec{y} \in Span(S)$.

iii. Let $c\in F$ and $\vec{x} \in Span(S)$. Then

$$c\vec{x} = c \sum_{i=1}^n a_i\vec{v_i} = \sum_{i=1}^n (ca_i)\vec{v_i}.$$
So $c\vec{x} \in Span(S)$.

## 1.10 Theorem

__Thm__ $V$ an $F$-v.s. and $S \subseteq V$. If $W$ is a subspace of $V$ and
$S$ is a subset of $W$, then $Span(S) \subseteq W$.

__Pf__ By HW.

## 1.11 Definition

__Def__ $V$ an $F$-v.s. If $V$ is spanned by a finite set, then $V$ is a 
finite-dimensional vector space. Otherwise, $V$ is an infinite dimensional 
vector space.

## 1.12 Definition

__Def__ An ordered set $S=\{\vec{v}_1,...,\vec{v}_n \}$ is called linearly 
independent if whenever

$$ a_1\vec{v}_1 + ... + a_n\vec{v}_n = \vec{0}, a_1 = ... = a_n = 0.$$

Otherwise, $S$ is linearly dependent.

## 1.13 Examples

a. $S = \{ sin^2(x), cos^2(x), 1 \} \subseteq \mathcal{C}^o([-\pi, \pi]).$ Linearly dependent because $a_1 = 1, a_2= 1, a_3=-1$.
b. $S = \{ 1,x,x^2 \} \subseteq \mathbb{C}[x]$. $a1 + bx + cx^2 = 0 \iff a=b=c=0$ by polynomial equality. So $S$ is linearly independent.
c. Consider $y^{\prime\prime} - 5y^{\prime} + 6y = 0$. Then solutions are of the form $y(t) = c_1e^{2t} + c_2e^{3t}, c_1,c_2 \in \mathbb{R}$. The solution set is equal to $Span(\{ e^{2t}, e^{3t} \})$.

## 1.14 Theorem

__Thm__ $V$ an $F$-v.s.

a. $\vec{0} \in S \implies S$ is linearly dependent.
b. $S = \{\vec{v}_1 \} \implies$ (S is linearly independent $\iff \vec{v} \neq \vec{0})$.
c. $S = \{ \vec{v}, \vec{w} \}$ is linearly independent $\iff \vec{v} \neq a\vec{w}, \forall a \in F$.

__Pf__ By Homework.

## 1.15 Theorem

__Thm__ Let $V$ an $F$-v.s., $S = \{\vec{v}_1, ..., \vec{v}_n \} \subseteq V$, and $m > 1$.

a. If $\vec{v}_1 \neq \vec{0}$, then $S$ is linearly dependent $\iff \exists j \in \{2,...,m \}$ such that $\vec{v}_j \in Span(\{v_1,...,v_{j-1} \}).$
b. If $\vec{v}_j \in Span(\{ \vec{v}_1, ..., \vec{v}_n \}), j \in \{2,...,m\},$
$$Span(S) = Span(S \setminus \{\vec{v}_j \}).$$

__Pf__ By Homeowork

## 1.18 Theorem
*9/1/22*

__Thm__ $V$ a f.d, $F$-v.s. and $V \neq \{\vec{0}\}$. Suppose $S = \{ \vec{v}_1,...\vec{v}_n \}$ is a non-empty spanning set for $V$ i.e. $V=Span(S)$. Then some subset of $S$ is a basis for $V$.

__Pf__ If $S$ is lin. ind., then we are done. Else $S$ is lin. dependent.

Cases
\begin{enumerate}
	\item[i] If $ \vec{v}_1 = \vec{0}$, then let $S_1 = S \setminus \{ \vec{v}_1 \}$.
\item[ii] Else, $\vec{v}_1 \neq \vec{0}$, and by Theorem 1.15 $\exists j \in \{2,\dots, m \}$ such that 
$$ \vec{v}_j \in Span\{\vec{v}_1, \dots,\vec{v}_{j-1} \}$$
and
$$Span(S) = Span(s \setminus \{ \vec{v}_j \})$$

Let $S_1 = S \setminus \{ \vec{v}_j \}$.
\end{enumerate}

If $S_1$ is lin. ind. stop. Otherwise, go-to cases for $S_1$. Eventually, this process will stop, because $V$ is f.d.
$$S_0 = S, S,_1, \dots$$
where
$$Span(S) = Span(S_1) = \dots$$
and $\# S_i = m - i$.

At each step, we remove one element. This process stops since $V$ is f.d. So we must have at least one non-zero vector in some spanning set $S_n$, the iteration we stop at. Thus we will have a lin. ind., spanning set $S_n$.

## 1.19 Theorem

__Thm__ Every f.d. $V \neq \{\vec{0}\}$ has a basis.

__Pf__ From the definition, there exists a spanning set $S$ for $V$. Use Theorem 1.18 to form reduce $S$ to a basis.

## 1.20 Theorem

__Thm__ Let $V$ f.d. v.s. and $\mathcal{B} \subseteq V$. 
$\mathcal{B}$ is a basis for $V \iff$ every vector in $V$ can be written as a lin. combo. of vectors in $\mathcal{B}$ in exactly one way.

__Pf__ By Homework.

## 1.21 Exchange \& Replacement Theorem

__Thm__ $V$ an $F$-v.s. $U,W \subseteq V$. $U = \{\vec{u}_1,\dots,\vec{u}_m\}$ and $W = \{\vec{w}_1,\dots,\vec{w}_n\}$.
If $U$ is lin. ind. and $V = Span(W)$, then $\#U \leq \#W$.

__Pf__ Let $S_0 = \{ \vec{w}_1, \dots, \vec{w}_n \}$. Since $\vec{u}_1 \in Span(S_0)$, we have that 

$$\{\vec{u}_1, \vec{w}_1, \dots, \vec{w}_n \} $$ 
is lin. dep. and $\vec{u}_1 \neq \vec{0}$.

By Theorem 1.15, we can remove a vector from $S_0$ and the span is unchanged.

$$Span\{\vec{u}_1, \vec{w}_1, \dots, \vec{w}_n \} = Span\{\vec{u}_1, \vec{w}_1, \dots, \vec{w}_{n-1} \}$$
after relabelling.

Let $S_1 = \{\vec{u}_1, \vec{w}_1, \dots, \vec{w}_{n-1} \}$. We can repeat this process to gain a sequence were 
$$S_j = \{ \vec{u}_1, \dots, \vec{u}_j, \vec{w}_1, \dots, \vec{w}_{n-j}\}$$.

By Theorem 1.15,

$$Span(S) = Span(S_0)= Span(S_1) = \dots = Span(S_{m-1}).$$

Consider 
$$S_{m-1} = \{ \vec{u}_1, \dots, \vec{u}_{m-1},\vec{w}_1, \dots, \vec{w}_{n-(m-1)}\}.$$

By Theorem 1.15, we can throw out a vector that is a lin. combo of the previous vectors, but $U$ is lin. ind., so it must be an element of $W$ that we throw out. Thus $n-(m-1) \geq 1 \implies n \geq m$.

## 1.22 Theorem

__Thm__ $U,V$ are $F$-v.s. with $U$ a subspace of $V$. Then if $V$ is f.d., then $U$ is f.d.

__Pf__ $V$ f.d. $\implies \exists W$ such that $W = \{ \vec{w}_1, \dots, \vec{w}_n \}$ and $V = Span(W)$.

If $U = \{ \vec{0} \}$, then $U = Span\{\vec{0}\}$, and we're done. Otherwise,
choose $\vec{u}_1 \in U, \vec{u}_1 \neq \vec{0}$. Let $S_1 = \{ \vec{u}_1 \}$.

Cases 
\begin{enumerate}
	\item[i] If $U = Span(S_1)$, then done.
	\item[ii] If $U \neq Span(S_1)$, choose $\vec{u}_2 \in U \setminus Span(S_1)$.
\end{enumerate}

Let $S_2 = \{ \vec{u}_1, \vec{u}_2 \}$. $S_2$ is lin. ind. by Theorem 1.15. Now, 
go-to cases using $S_2$.

Note $\# S_i = i$ and $i \leq m$ by Theorem 1.21. So this sequence must terminate.

Thus $U$ is finite dimensional.

## 1.23 Theorem

__Thm__ $V$ a f.d. $F$-v.s. and $S \subseteq V$ a lin. ind. set. Then there exists a basis $\mathcal{B}$ for $V$ s/t $S \subset \mathcal{B}$ i.e. $S$ can be extended to a basis.

__Pf__ By Homework.

## 1.24 Theorem

__Thm__ $V$ a f.d. $F$-v.s. If $\mathcal{B}_1,\mathcal{B}_2$ are bases for $V$, then $\#\mathcal{B}_1 = \# \mathcal{B}_2$. 

__Pf__ Let $\mathcal{B}_1,\mathcal{B}_2$ be bases for $V$.  
By Theorem 1.21 $\# \mathcal{B}_1 \leq \# \mathcal{B}_2$ and 
$\# \mathcal{B}_2 \leq \# \mathcal{B}_1$ 
so $\# \mathcal{B}_1 = \# \mathcal{B}_2$ .

## 1.25 Def

__Def__ the dimension of a f.d. $F$-v.s. is the number of elements in any basis for $V$. If $V = \{ \vec{0} \}$, then define $dimV = 0$.

## 1.26 Theorem

__Thm__ Let $V$ be a non-trivial f.d. $F$-v.s. with dimension $n$.

\begin{enumerate}
	\item If $U$ is a s.s. of $V$, then $dimU \leq dimV$.
	\item If $S \subseteq V$ is a l.i. and $\# S =n$, then $S$ is a basis for $V$.
	\item If $S \subseteq V$ and $V = Span(S)$, then $\# S = n \implies S$ is a basis.
\end{enumerate}

__Pf__ Homework.

# 2 Linear Maps

## 2.1 Definition

__Def__ Let $V,W$ be $F$-v.s. An $F$-linear map from $V$ to $W$ is a map written 
$T : V \rightarrow W$ such that:
\begin{enumerate}
\item $T(\vec{x} + \vec{y}) = T(\vec{x}) + T(\vec{y}), \forall \vec{x},\vec{y} \in V$
\item $T(c\vec{x}) = cT(\vec{x}), \forall \vec{x} \in V, \forall c \in F$.
\end{enumerate}
(i) is additivity
(ii) is homogeneity of degree 1

## 2.2 Examples

\begin{enumerate}
\item Let $C^1(\mathbb{R}) = \{ f : \mathbb{R} \rightarrow \mathbb{R} \mid f^{\prime} \text{ exists and } f^\prime \in C^0(\mathbb{R})$. In general $C^k(\mathbb{R}) = \{ f : \mathbb{R} \rightarrow \mathbb{R} \mid f^{(k)} \text{ exists and } f^{\prime} \in C^{k-1}(\mathbb{R})\}$.
This is linear:
$$\frac{d}{dx} : C^k(\mathbb{R}) \rightarrow C^{k-1}(\mathbb{R})$$
$$ f \mapsto \frac{df}{dx}$$
\begin{enumerate}
\item $$\frac{d}{dx}(f+g) = \frac{df}{dx} + \frac{dg}{dx}$$
\item $$\frac{d}{dx}(cf) = c \frac{df}{dx} $$
\end{enumerate}

\item Consider:
$$ \int : C^0([a,b]) \rightarrow \mathbb{R}$$
$$ f \mapsto \int_a^b f(x) dx$$
This is linear:
\begin{enumerate}
\item $$\int_a^b f(x) + g(x) dx = \int_a^b f(x)dx + \int_a^b g(x)dx$$ 
\item $$\int_a^b cf(x)dx = c\int_a^b f(x)dx$$ 
\end{enumerate}

\item Consider 
$$L: C^2(\mathbb{R}) \rightarrow C^0(\mathbb{R})$$
$$f \mapsto f^{\prime\prime} + k^2f$$
This is linear. (Check!)

\item $$T : \mathbb{R}^3 \rightarrow \mathbb{R}^2$$
$$\begin{bmatrix} a \\ b \\ c \end{bmatrix} \mapsto \begin{bmatrix} a+b-c \\ c \end{bmatrix}$$ 
Check if linear:
\begin{enumerate}
\item \[
T(
\begin{bmatrix}
a_1 + a_2 \\ b_1 + b_2 \\ c_1 + c_2	
\end{bmatrix}
)
=
\begin{bmatrix}
a_1 +a_2 +b_1 + b_2 - c_1 -c_2 \\
c_1 + c_2
\end{bmatrix}
.\]
\[
=
\begin{bmatrix}
a_1 + b_1 - c_1 \\
c_1
\end{bmatrix}
+
\begin{bmatrix}
a_2 + b_2 -c_2 \\
c_2
\end{bmatrix}
=
T(
\begin{bmatrix}
a_1 \\
b_1\\
c_1\\
\end{bmatrix}
)
+
T(
\begin{bmatrix}
a_2 \\
b_2\\
c_2\\
\end{bmatrix}
)
.\] 
\item \[
T(
\begin{bmatrix}
	sa \\ sb \\ sc
\end{bmatrix})
= 
\begin{bmatrix}
sa + sb - sc \\
sc
\end{bmatrix}
=
sT(
\begin{bmatrix}
a \\ b \\ c
\end{bmatrix}
)
.\] 
\end{enumerate}

\item Now consider:
$$T : \mathbb{R}^3 \mapsto \mathbb{R}^2$$
$$\begin{bmatrix} a \\ b\\ c \end{bmatrix}\mapsto \begin{bmatrix}ab-c \\ c \end{bmatrix}$$
Check if linear (Spoiler, it's not):
\begin{enumerate}
\item
\item \[
T(
\begin{bmatrix}
sa \\ sb \\ sc
	
\end{bmatrix}
)
=
\begin{bmatrix}
s^2ab - sc \\
sc
\end{bmatrix}
=
s
\begin{bmatrix}
sab - c \\
c
\end{bmatrix}
.\]  
\end{enumerate}

\item Let
$$ L : \mathbb{C}[x] \rightarrow \mathbb{C}[x] $$
$$ f \mapsto x\cdot f$$
Check if linear:
\begin{enumerate}
\item Let $f,g \in \mathbb{C}[x]$ with
$$f = \sum_{i=0}^n a_ix_i \text{ and } g=\sum_{i=0}^m b_ix_i$$
WLOG, let $m \leq n$ and define $b_i = 0$ for $m < i \leq n$.
\[
L(f+g) = L(\sum_{i=0}^n (a_i + b_i)x_i) = x \sum_{i=0}^n (a_i + b_i) x^i = \sum_{i=0}^n (a_i + b_i) x^{i+1}
.\]
\[
= \sum_{i=0}^n a_i x^{i+1} + \sum_{i=0}^n b_i x^{i+1} = \sum_{i=0}^n a_i x^{i+1} + \sum_{i=0}^m b_i x^{i+1}
.\] 
\item
\end{enumerate}
\end{enumerate}

## 2.3 Theorem 

__Thm__ $T : V \rightarrow W$ linear. Then $T(\vec{0}) = \vec{0}$.

__Pf__ $T(\vec{0}) = T(\vec{0} + \vec{0}) = T(\vec{0}) + T(\vec{0})$.
$\implies \vec{0} =  T(\vec{0})$.

## 2.4 Definition

__Def__ $T : V \rightarrow W$ linear. Define

$$Ker(T) = \{ \vec{v} \in V \mid T(\vec{v}) = \vec{0} \},$$
called the kernal (null space) of $T$.

## 2.5 Theorem

__Thm__ $T : V \rightarrow W$ linear. Then $Ker(T)$ is a s.s. of $V$.

__Pf__ 
\begin{enumerate}
\item Theorem 2.3, $T(\vec{0}) = \vec{0} \implies \vec{0} \in Ker(T)$.

\item Let $\vec{x}, \vec{y} \in Ker(T)$. Want to show that $\vec{x} + \vec{y} \in Ker(T)$.
By definition of $Ker(T)$
$$ T(\vec{x}) = 0$$
$$ T(\vec{y}) = 0.$$
Now 
$$ T(\vec{x} + \vec{y}) = T(\vec{x}) + T(\vec{y}) = \vec{0} + \vec{0} = \vec{0}.$$
$$\implies \vec{x} + \vec{y} \in Ker(T).$$

\item Let $\vec{x}\in Ker(T)$, $c \in F$.

$$ T(c \vec{x}) = c T(\vec{x}) = c \cdot \vec{0} = \vec{0}$$
$$\implies c\vec{x} \in Ker(T).$$

\end{enumerate}

## 2.6 Definition

__Def__ a function $f : X \rightarrow Y$ is called injective if 
$$ f(x) = f(y) \implies x=y, \forall x,y \in X.$$

## 2.7 Theorem
*9-8-22*

__Thm__ $T : V \rightarrow W$ linear. $T$ is injective $\iff Ker(T) = \{ \vec{0} \}$.

__Pf__ Assume $T$ is injective. Want to show that 
$$Ker(T) = \{ \vec{0} \}.$$
Let $\vec{u} \in Ker(T)$. Then $T(\vec{u}) = \vec{0}$ and $T(\vec{0}) = \vec{0}$ by 2.3.
Thus $T(\vec{u}) = T(\vec{0})$ and $T$ injective gives $\vec{u} = \vec{0}$. Also, by 2.5, $Ker(T)$ is a s.s. so non-empty.

Now assume $Ker(T) = \{0 \}.$ WTS that $T$ is injective.
Let $\vec{x},\vec{y} \in V$ be such that 
$$T(\vec{x}) = T(\vec{y})$$
$$T(\vec{x}) - T(\vec{y}) = \vec{0}$$
Since the kernal of $T$ is trivial.
$$\vec{x} - \vec{y} = \vec{0}$$
$$\implies \vec{x} = \vec{y}$$

## 2.8 Definition

__Def__ Let $f X : \rightarrow Y$ be a map, then the range of $f$ is 
$$range(f) =\{f(x) \mid x \in X \}$$

## 2.9 Theorem 

__Thm__ $T : V \rightarrow W$ $F$-linear, then range$(T)$ is a subspace of $W$.

__Pf__ By Homework

## 2.10 Definition 

__Def__ $f: X \rightarrow Y$ is called surjective if 
$$range(f) = Y.$$

## 2.11 Definition 

__Def__ $T : V \rightarrow W$ is called an $F$-isomorphism if: 
\begin{enumerate}
\item $T$ is $F$-linear.
\item $T$ is injective.
\item $T$ is surjective.
\end{enumerate}

We say that $V$ and $W$ are $F$-isomorphic written TODO if there exists an 
$F$-isomorphism between them.

## 2.12 DEF

__Def__ $T : V \rightarrow W$ F-linear. Define 
$$rank(T) = dim(range(T)),$$
and define 
$$nullity(T) = dim(Ker(T)).$$

## 2.13 Theorem (Rank-Nullity)

__Thm__ $T: V \rightarrow W$ F-linear and $V$ f.d. Then 
$$dimV = rank(T) + nullity(T).$$

__Pf__ Know that $Ker(T)$ is a s.s. of $V$. So, $Ker(T)$ is finite dimensional by Theorem 1.22.
So there exists $U= \{\vec{u}_1, \dots, \vec{u}_m \}$ where $U$ is a basis for $Ker(T)$.

So extend $U$ to be a basis for $V$ and call it
$$
\mathcal(B)_V = \{ \vec{u}_1, \dots, \vec{u}_m, \vec{v}_1, \dots, \vec{v}_n \}.
.$$

Note $dimV = m +n$ and $nullityT = m.$
Claim: $rank(T) = n$. Idea, show that $S = \{ T(\vec{v}_1), \dots, T(\vec{v}_n) \}$ is a basis for range($T$).
Let $T(\vec{v}) \in range(T)$ be an arbitrary vector with $\vec{v}\in V$. Since, $\vec{v} \in V,$
$$ \vec{v} = a_1\vec{u}_1 + \dots + a_m \vec{u}_m + b_1\vec{v}_1 + \dots +  b_n\vec{v}_n$$
Then:
$$T(\vec{v}) = a_1T(\vec{u}_1) + \dots + a_m T(\vec{u}_m) + b_1T(\vec{v}_1) + \dots +  b_nT(\vec{v}_n)$$
$$ = b_1 T(\vec{v}_1) + \dots b_n T(\vec{v}_n).$$
So $S$ is a spanning set for $range(T).$ To show linear independence, let 

$$ c_1T(\vec{v}_1) + \dots + c_nT(\vec{v}_n) = \vec{0}.$$
$$T(c_1\vec{v}_1 + \dots + c\vec{v}_n) = \vec{0}$$
$$\implies T(c_1\vec{v}_1 + \dots + c\vec{v}_n)  \in Ker(T)$$
$$\implies c_1\vec{v}_1 + \dots + c\vec{v}_n = d_1\vec{u}_1 + \dots + d_n\vec{u}_n$$
Thus 
$$c_1\vec{v}_1 + \dots + c\vec{v}_n - d_1\vec{u}_1 + \dots + d_n\vec{u}_n = \vec{0}$$
$$ \implies d_1 = \dots = d_m = c_1 = \dots c_n = 0$$
Since $\mathcal{B}_V$ is a basis.
This implies that $dim(range(T)) = n = rank(T)$. So

$$dim V = m + n.$$
$$rank(T) = n.$$
$$nulltiy(T) = m.$$

## 2.14 Theorem

__Thm__ $V$ an $F$-vs with basis $\{ \vec{v}_1, \dots, \vec{v}_n \}$ and $W$ 
an F-v.s. with basis $\{ \vec{w}_1, \dots, \vec{w}_n \}$.
There exists a unique $F$-linear map $T : V \rightarrow W$ such that 
$$T(\vec{v}_j) = \vec{w}_j, \forall j \text{ with } 1 \leq j \leq n.$$ 

__Pf__ First, existence of map $T$.

Let $\vec{v} \in V$ be arbitrary. Then $\vec{v} = c_1\vec{v}_1 + \dots + c_n\vec{v}_n$ for some $c_1,\dots, c_n \in F.$
Define $T(\vec{v})$ by 

$$T(\vec{v}) = c_1\vec{w}_1 + \dots + c_n \vec{w}_n.$$

Since $\{\vec{w}_1, \dots, \vec{w}_n \}$ is a basis. 

$$c_1\vec{w}_1 + \dots + c_n\vec{w}_n$$

describes a unique vector. So $T$ is well-defined.

If $c_j= 1$ and $c_i=0 \forall i$ with $1 \leq i \leq n$ and $i \neq j$
then 

$$T(c_1\vec{v}_1 + \dots +c_n \vec{v}_n) = T(\vec{v}_j) = \vec{w}_j$$

So our map works how we want it to.

LINEARITY AND UNIQUENESS BY HOMEWORK

## 2.15 theorem

__Thm__ $V,W$ f.d. $F$-v.s.

$$V \cong W \iff dim(V) = dim(W)$$

__Pf__ Assume $V \cong W$ thus $\exists T : V \rightarrow W$ F-isom.
Recall $dimV = rankT + nullityT$. 

$T$ isomorphic $\implies$ T inj $\implies Ker(T) = \{\vec{0} \} \implies dim(Ker(T)) = 0 \implies nullity(T) = 0.$

So $dimV = rank(T)$. 

By $T$ is surjective $\implies range(T) = W \implies dim(range(T)) = dimW$.

So dim$V=$dim$W$.

Now, assume that dim$V=$dim$W$. 
Let $V$ have a basis $\{\vec{v}_1, \dots, \vec{v}_n \}$ and $W$ basis $\{ \vec{w}_1, \dots, \vec{w}_n \}$.
By 2.14, there exists a unique $T : V \rightarrow W$ such that $T(\vec{v}_j = \vec{w}_j)$
for $1\leq j \leq n$.

Need to show $T$ is $F$-isom. $T$ is $F$-linear by 2.14.
\begin{enumerate}
\item[Surjectivity] Let $\vec{w} \in W$ be written as:
$$\vec{w} = c_1\vec{w}_1 + \dots + c_n\vec{w}_n.$$
Then 
$$T(c_1\vec{v}_1 + \dots + c_n\vec{v}_n) = \vec{w}.$$
\item[Injectivity] Let $x \in V$ and assume $T(\vec{x})= \vec{0}$. 
$$\vec{x} = a_1\vec{v}_1 + \dots + a_n\vec{v}_n$$
and
$$T(\vec{x}) = a_1\vec{w}_1 + \dots + a_n\vec{w}_n = \vec{0}$$
Since $\{\vec{w}_1, \dots,\vec{w}_n \}$ we must have $a_1 + \dots = a_n = 0$, so
$\vec{x} = \vec{0}$, thus $T$ is injective by Theorem 2.6,
and $T$ is an $F$-isomorphism as desired.
\end{enumerate}

## 2.16 Corollary

$V$ a f.d. $F$-v.s. with dim$V$=n. Then $V \cong F^n$.

__Pf__ $F^n$ is $n$-dimensional. Use previous Theorem.

## 2.17 Definition

__Def__ Define a map $I : V \rightarrow V$ by $\vec{v} \mapsto \vec{v}$, called the identity map on $V$.

## 2.18 Defintion

__Def__ Let $T : V \rightarrow W$ be linear. We say $T$ is invertible if 
$$\exists S : W \rightarrow V \text{ linear such that }$$
\begin{enumerate}
\item[(i)] $$S \circ T = I_V$$
\item[(ii)]$$ T \circ S = I_W$$

We call $S$ an inverse of $T$.
	
\end{enumerate}

## 2.19 Theorem

__Thm__ If $T: V \rightarrow W$ linear has an inverse, then that inverse is unique.

__Pf__ Let $S_1$ and $S_2$ be inverses of $T$, some invertible linear map. We
have 

$$ S_1 \circ T = S_2 \circ T = I_V$$
$$T \circ S_1 = T \circ S_2 = I_W$$
$$S_1 = S_1 \circ I_W = S_1 \circ (T \circ S_2) = (S_1 \circ T) \circ S_2 = 
I_V \circ S_2 = S_2.$$

## 2.20 Definition

__Def__ If $T$ has an inverse, denote it $T^{-1}$.

## 2.21 Theorem

__Thm__ $T : V \rightarrow W$ is invertible if and only if $T$ is an $F$-isomorphism.

__Pf__ Assume $T$ is invertible. Then there exists a unique map $T^{-1}$ such that
$$T^{-1} \circ T = I_V.$$

\begin{enumerate}
\item[i] $T$ is linear by assumption.

\item[ii] Let $T(\vec{x}) = T(\vec{y})$. then
$$T^{-1} \circ T (\vec{x}) = T^{-1} \circ T (\vec{y})$$
$$I(\vec{x}) = I(\vec{y})$$
$$\vec{x} = \vec{y}$$

\item[iii] Let $\vec{w} \in W$ be arbitrary, then $T \circ T^{-1}(\vec{w}) = \vec{w}$.
	
\end{enumerate}

So $T$ is an $F$-isomorphism.

Now, let $T$ be an $F$-isomorphism. We must construct $S : W \rightarrow V$ 
linear such that

$$T \circ S = I_W,$$
$$ S \circ T = I_V.$$

Since $T$ is both surjective and injective, we know that $\forall \vec{w} \in W,
\exists \vec{v} \in V$ such that $T(\vec{v}) = \vec{w},$
and $T(\vec{x}) = T(\vec{y}) \implies \vec{x} = \vec{y}$. So define
$$S : W \rightarrow V$$
$$\vec{w} \rightarrow \vec{v}.$$
where $T(\vec{v}) = \vec{w}$.
Every $\vec{w}$ has such a $\vec{v}$ by surjectivity of $T$ and that $\vec{v}$
is unique by injectivity of $T$.

Now compute:
$$T \circ S(\vec{w}) = T(\vec{v}) = \vec{w} \implies T \circ S = I_W$$
$$S \circ T(\vec{v}) = S (\vec{w}) = \vec{v} = I_V$$

The linearity of $S$ follows from the linearity of $T$. 
\begin{enumerate}

	\item[(+)]
		If $S(\vec{w}_1) = \vec{v}_1, S(\vec{w}_2) = \vec{w}_2,$
		$$S(\vec{w}_1 + \vec{w}_2) = S(T(\vec{v}_1) + \vec{v}_2) = \vec{v}_1 + \vec{v}_2 
		= S(\vec{w}_1) + S(\vec{w}_2)$$
	\item[($\cdot$)]
\end{enumerate}

Similarly for homogeneity.

# 3 Matrices

## 3.1 Definition

__Def__ Let $m,n \in \mathbb{N}, F$ a field. An $m \times n$ matrix over $F$ is
a grid with $m$ rows and $n$ columns consisting of elements of $F$.
$$
\begin{bmatrix}
a_{11} & a_{12} & \dots & a_{1n}\\
a_{21} & a_{22} & \dots & a_{2n}\\
\vdots & \vdots & \ddots & \vdots \\
a_{m_1} & a_{m_2} & \ldots & a_{mn} \\
\end{bmatrix}
$$

We denote the space of all such matrices as $M_{m\times n}(F)$

## 3.2 

__Def__ $V$ an $F$-v.s. with basis $\mathcal{B} = \{ \vec{v}_1, \ldots , \vec{v}_n \}$
and $F^n$ with basis $\{\vec{e}_1, \dots, \vec{e}_n \},$ the standard basis.

Then $V \cong F^n$ by Corollary 2.16 and by Theorem 2.14, $\exists T : V \rightarrow F^n$
an $F$-isom with $T(\vec{v}_i) = \vec{e}_i$ and 

$$T(c_1\vec{v}_1 + \dots + c_n\vec{v}_n) = c_1\vec{e}_1  + \dots + c_n\vec{e}_n = 
\begin{bmatrix}
c_1 \\
\vdots \\
c_n
\end{bmatrix}
$$
Define $[ c_1\vec{v}_1 + \dots + c_n\vec{v}_n ]_{\mathcal{B}} = \begin{bmatrix}
c_1 \\
\vdots \\
c_n
\end{bmatrix} \in F^n,$

called the coordinate vector of $c_1\vec{v}_1 + \dots + c_n\vec{v}_n$ with 
respect o the basis $\mathcal{B}$.


## 3.3 Examples

\item $P_2(\mathbb{R}), \mathcal{B} = \{1,x,x^2 \}$.
$$[x^2 + x +1 ]_{\mathcal{B}} = 
\begin{bmatrix}
1 \\
1\\
1\\
\end{bmatrix}$$	

## 3.4 Definition

Let $T : V \rightarrow W$ be linear with $\mathcal{B} = \{ \vec{v}_1, \dots, \vec{v}_n \}$
and $\mathcal{\vec{w}, \dots, \vec{w}_m}$ bases for $V$ and $W$ respectively.
Then for $\vec{v} \in V,$

$$T(\vec{v}) = T(C_1\vec{v}_1 + \dots +c_n \vec{v}_n)$$
$$=c_1T(\vec{v}_1) + \dots + c_n T(\vec{v}_n)$$

where $T(\vec{v}_i) \in W$. So 

$$[T(\vec{v}_i)]_{\mathcal{C}} = \begin{bmatrix}
a_1i \\
\vdots \\
a_mi \\
\end{bmatrix}$$

So:

$$[T(\vec{v})]_{\mathcal{C}} = c_1 \begin{bmatrix} a_{11} \\ \vdots \\ am_1 \end{bmatrix} + \dots + c_n \begin{bmatrix} a_{1n} \\ \vdots \\ a{mn} \\ \end{bmatrix}$$

Define $A_{\mathcal{B}}^{\mathcal{C}} = \begin{bmatrix}
a_{11} & \dots & a_{1n} \\
\vdots & \ddots & \vdots \\
a_{m_1} & \dots & a_{mn}
\end{bmatrix} \in M_{m\times n }(F),$

the matrix of $T$ with respect to the bases $\mathcal{B}$ and $\mathcal{C}$.
