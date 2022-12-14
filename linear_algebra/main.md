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

# 1 Vector Spaces

*8/25/22*


## 1.1 Note

## 1.2 Definition: Vector Space

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

## 1.3 Todd-given Examples

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

## 1.5 Definition: Vector Subspace

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
3. $\mathcal{C}^o(\mathbb{R}) = \left\{ f: \mathbb{R} \rightarrow \mathbb{R}, \text{ $f$ continuous} \right\}$.
4. For any $F$-v.s. $V$, ${\vec{0}}$ and $V$ are subspaces.

## 1.8 Definition: Span
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

## 1.11 Definition: Finite Dimensional & Infinite Dimensional

__Def__ $V$ an $F$-v.s. If $V$ is spanned by a finite set, then $V$ is a 
finite-dimensional vector space. Otherwise, $V$ is an infinite dimensional 
vector space.

## 1.12 Definition: Linearly Independent & Linearly Dependent

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

## 1.25 Definition: Dimension of a Vector Space

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

## 2.1 Definition: $F$-Linear Map

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

## 2.4 Definition: Kernel

__Def__ $T : V \rightarrow W$ linear. Define

$$Ker(T) = \{ \vec{v} \in V \mid T(\vec{v}) = \vec{0} \},$$
called the kernel (null space) of $T$.

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

## 2.6 Definition: Injectivity

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

## 2.8 Definition: Range (or Image) of a map 

__Def__ Let $f : X \rightarrow Y$ be a map, then the range of $f$ is 
$$range(f) =\{f(x) \mid x \in X \}$$

## 2.9 Theorem 

__Thm__ $T : V \rightarrow W$ $F$-linear, then range$(T)$ is a subspace of $W$.

__Pf__ By Homework

## 2.10 Definition: Surjectivity

__Def__ $f: X \rightarrow Y$ is called surjective if 
$$range(f) = Y.$$

## 2.11 Definition: $F$-Isomorphism 

__Def__ $T : V \rightarrow W$ is called an $F$-isomorphism if: 
\begin{enumerate}
\item $T$ is $F$-linear.
\item $T$ is injective.
\item $T$ is surjective.
\end{enumerate}

We say that $V$ and $W$ are $F$-isomorphic, written $V \cong W$, if there exists an 
$F$-isomorphism between them.

## 2.12 Definition: Rank and Nullity

__Def__ $T : V \rightarrow W$ F-linear. Define 
$$\text{rank}(T) = \text{dim}(\text{range}(T)),$$
and define 
$$\text{nullity}(T) = \text{dim(Ker}(T)).$$

## 2.13 Theorem (Rank-Nullity)

__Thm__ $T: V \rightarrow W$ F-linear and $V$ f.d. Then 
$$\text{dim}V = \text{rank}(T) + \text{nullity}(T).$$

__Pf__ Know that $\text{Ker}(T)$ is a s.s. of $V$. So, $\text{Ker}(T)$ is finite dimensional by Theorem 1.22.
So there exists $U= \{\vec{u}_1, \dots, \vec{u}_m \}$ where $U$ is a basis for $Ker(T)$.

So extend $U$ to be a basis for $V$ and call it
$$
\mathcal(B)_V = \{ \vec{u}_1, \dots, \vec{u}_m, \vec{v}_1, \dots, \vec{v}_n \}.
.$$

Note $\text{dim}V = m +n$ and $\text{nullity}T = m.$
Claim: $\text{rank}(T) = n$. Idea, show that $S = \{ T(\vec{v}_1), \dots, T(\vec{v}_n) \}$ is a basis for range($T$).
Let $T(\vec{v}) \in \text{range}(T)$ be an arbitrary vector with $\vec{v}\in V$. Since, $\vec{v} \in V,$
$$ \vec{v} = a_1\vec{u}_1 + \dots + a_m \vec{u}_m + b_1\vec{v}_1 + \dots +  b_n\vec{v}_n$$
Then:
$$T(\vec{v}) = a_1T(\vec{u}_1) + \dots + a_m T(\vec{u}_m) + b_1T(\vec{v}_1) + \dots +  b_nT(\vec{v}_n)$$
$$ = b_1 T(\vec{v}_1) + \dots b_n T(\vec{v}_n).$$
So $S$ is a spanning set for $\text{range}(T).$ To show linear independence, let 

$$ c_1T(\vec{v}_1) + \dots + c_nT(\vec{v}_n) = \vec{0}.$$
$$T(c_1\vec{v}_1 + \dots + c\vec{v}_n) = \vec{0}$$
$$\implies T(c_1\vec{v}_1 + \dots + c\vec{v}_n)  \in Ker(T)$$
$$\implies c_1\vec{v}_1 + \dots + c\vec{v}_n = d_1\vec{u}_1 + \dots + d_n\vec{u}_n$$
Thus 
$$c_1\vec{v}_1 + \dots + c\vec{v}_n - d_1\vec{u}_1 + \dots + d_n\vec{u}_n = \vec{0}$$
$$ \implies d_1 = \dots = d_m = c_1 = \dots c_n = 0$$
Since $\mathcal{B}_V$ is a basis.
This implies that $\text{dim(range}(T)) = n = \text{rank}(T)$. So

$$\text{dim} V = m + n.$$
$$\text{rank}(T) = n.$$
$$\text{nulltiy}(T) = m.$$

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

## 2.15 Theorem

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

## 2.17 Definition: Identity Map

__Def__ Define a map $I : V \rightarrow V$ by $\vec{v} \mapsto \vec{v}$, called the identity map on $V$.

## 2.18 Definition: Invertible Linear Map

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

## 2.20 Definition: Inverse of A Linear Map

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


# 3 Matrices: a convenient representation of a linear map between vector spaces for a given choice of bases for said vector spaces

## 3.1 Definition: Matrix as an Object

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

## 3.2 Definition: Coordinate Vector in $F^n$

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

1. $P_2(\mathbb{R}), \mathcal{B} = \{1,x,x^2 \}$.
$$[x^2 + x +1 ]_{\mathcal{B}} = 
\begin{bmatrix}
1 \\
1\\
1\\
\end{bmatrix}$$	

## 3.4 Definition: The Matrix of a Linear Transformation

Let $T : V \rightarrow W$ be linear with $\mathcal{B} = \{ \vec{v}_1, \dots, \vec{v}_n \}$
and $\mathcal{C} \{ = \vec{w}, \dots, \vec{w}_m \}$ bases for $V$ and $W$ respectively.
Then for $\vec{v} \in V,$

$$T(\vec{v}) = T(c_1\vec{v}_1 + \dots +c_n \vec{v}_n)$$
$$=c_1T(\vec{v}_1) + \dots + c_n T(\vec{v}_n)$$

where $T(\vec{v}_i) \in W$. So 

$$[T(\vec{v}_i)]_{\mathcal{C}} = \begin{bmatrix}
a_{1i} \\
\vdots \\
a_{mi} \\
\end{bmatrix}$$

So:

$$[T(\vec{v})]_{\mathcal{C}} = 
c_1 \begin{bmatrix} a_{11} \\ \vdots \\ a_{m1} \end{bmatrix} 
+ \dots 
+ c_n \begin{bmatrix} a_{1n} \\ \vdots \\ a_{mn} \\ \end{bmatrix}$$

Define $A_{\mathcal{B}}^{\mathcal{C}} = \begin{bmatrix}
a_{11} & \dots & a_{1n} \\
\vdots & \ddots & \vdots \\
a_{m_1} & \dots & a_{mn}
\end{bmatrix} \in M_{m\times n }(F),$

the matrix of $T$ with respect to the bases $\mathcal{B}$ and $\mathcal{C}$.

*9-15-22*

$$[T]_{\mathcal{B}}^{\mathcal{B}}$$


## 3.5 Definition: $A\vec{v}$

__Def__ Let $A \in M_{mn}(F)$ where the columns are given by: $\vec{a}_1, \dots, \vec{a}_n$.
\begin{enumerate}

	\item we write $A = [\vec{a}_1 \ldots \vec{a}_n] = \begin{bmatrix} 
    a_{11} 		& \ldots & a_{1n} \\
	\vdots		&\ddots	&\vdots	\\
	a_{m_1} & \ldots		&	a_{mn}
	\end{bmatrix}$ 
	
	\item if $\vec{v} \in \mathbb{R}^n$ with $\vec{v} = \begin{bmatrix} 
 	v_1 \\
	\vdots \\
	v_n \\
	\end{bmatrix}$,
	then define the product $A\vec{v}$ by

$$A\vec{v}= v_1\vec{a}_1 + \dots + v_{n}\vec{a}_n.$$
	
\end{enumerate}

## 3.5 $\alpha$ Example

Let $$A = \begin{bmatrix} 
1 & 0 & 3 \\
1 & 2 & -1 \\
\end{bmatrix}, \vec{v} = \begin{bmatrix} 
1 \\
0\\
2
\end{bmatrix} 
$$
Then 

$$A\vec{v} = 1 \cdot \begin{bmatrix} 
1\\
1
\end{bmatrix} +0 \cdot \begin{bmatrix} 
0 \\
2\\
\end{bmatrix} + 2 \cdot \begin{bmatrix} 
3 \\
-1 
\end{bmatrix} 
$$
 
## 3.6 Theorem

__Thm__ $A \in M_{mn}(F)$ then $A$ is a linear map 
$$ A : \mathbb{R}^n \rightarrow \mathbb{R}^m $$
with
$$ \vec{x} \mapsto A\vec{x}$$

__Pf__ Homework.

## 3.6 $\beta$ Example

$A$ is a map 
$$A : \mathbb{R}^3 \rightarrow \mathbb{R}^2$$
$$\begin{bmatrix} 
a\\
b\\
c\\
\end{bmatrix} 
\mapsto
a\begin{bmatrix} 
1\\1 
\end{bmatrix} 
+
b\begin{bmatrix} 
0\\
2
\end{bmatrix} 
+
c\begin{bmatrix} 
3\\
1
\end{bmatrix} 
\in \mathbb{R}^3
$$



## 3.7 Theorem

__Thm__ Let $T: V \rightarrow W$ be linear. dim$V =n$, dim$W=m$, and choose basis $\mathcal{B}= \{v_1, \dots, \vec{v}_n \}$ for $V$ and basis $\mathcal{C} = \{\vec{w}_1, \dots, \vec{w}_n\}$ for $W$.

Then
$$[T(\vec{v})]_{\mathcal{C}} = [T]_{\mathcal{B}}^{\mathcal{C}}[\vec{v}]_{\mathcal{B}}$$

__TODO add a beautiful diagram__

__Pf__ By homework.

## 3.7 $\gamma$ Example

Consider:

$$ T: \mathbb{P}^2(\mathbb{R}) \rightarrow \mathbb{P}(\mathbb{R})$$
$$ ax^2 +bx +c \mapsto 2ax + b$$

Choose

$$\mathcal{B} = \{ 1,x,x^2 \}$$
$$\mathcal{C} = \{1,1+x\}.$$

## 3.8 Definition: Scaling a Matrix, Adding Matrices

__Def__ Suppose $A,B \in M_{mn}(F)$ and $c \in F$. Define
\begin{enumerate}
	\item $cA = [c\vec{a}_1 \ldots c\vec{a}_n].$
	\item $A + B = [(\vec{a}_1 + \vec{b}_1) \dots (\vec{a}_n + \vec{b}_n)]$
\end{enumerate}

So $M_{mn}(F)$ is an $F$-v.s (note this is a consequence of 3.6).

## 3.9 Theorem

__Thm__ The space $M_{mn}(F)$ has dimension $m\cdot n$ as an $F$-v.s.

__Pf__ by Homework.

## 3.10 Theorem

__Thm__  The composition of linear maps is linear.

__Pf__  Hw.

## 3.11 Definition: Matrix Multiplication 

__Def__ Let $A \in M_{mn}(F)$ and $B \in M_{np}(F)$. Then define $AB$ by:

$$AB : \mathbb{R}^p \rightarrow \mathbb{R}^m$$
$$\vec{v} \mapsto A(B\vec{v})$$

Well-defined map with a matrix representation by definition of composition of 
functions and Theorem 3.10.

## 3.12 Theorem

__Thm__ $A \in M_{mn}(F), B \in M_{np}(F)$. Then $AB = [A\vec{b}_1 \dots A\vec{b}_p].$

__Pf__ Let $\vec{v}\in \mathbb{R}^p$ with $$\vec{v} = \begin{bmatrix} 
v_1\\
\vdots\\
v_p
\end{bmatrix} $$

then $AB(\vec{v})= A(B\vec{v})$.

$$A(v_1\vec{b}_1 + \dots v_p \vec{b}_p)$$
$$= v_1A\vec{b}_1 + \dots + v_p A \vec{b}_p$$
$$= v_1(A\vec{b}_1) + \dots + v_p(A\vec{b}_p)$$
$$ = [A\vec{b}_1 \dots A\vec{b}_p] \begin{bmatrix} 
v_1\\
\vdots\\
v_p\\
\end{bmatrix} = [A\vec{b}_1 \dots A \vec{b}_p]\vec{v}.$$

## 3.12 $\delta$ Example

Omitted because I didn't type fast enough ??\\\_(._.)_/??

## A Summary of Chapter 3 Thus Far.
*9-20-22*

$A_{m\times n}$, $\vec{x} \in \mathbb{R}^n$
$$A\vec{x} = x_1\vec{a}_1 + \ldots + x_n\vec{a}_n$$
$$A_{mn}, B_{mp} = [A\vec{b}_1 \ldots A\vec{b}_p]$$

$T: V \rightarrow W$, $\mathcal{B}$ a basis for $V$. $\mathcal{C}$ a basis for $W$.

$$[T]_{\mathcal{B}}^{\mathcal{C}} = [[T(\vec{v}_1)]_{\mathcal{C}} \ldots [T(\vec{v}_n)]_{\mathcal{C}}]$$

## 3.13 Theorem

__Thm__ Let $V, W, Z$ be vector spaces with bases $\mathcal{A}, \mathcal{B}, \mathcal{C}$ 
and dimensions $p,n,m$ respectively.

If $T: V \rightarrow W$ and $S: W \rightarrow Z$ are linear, then

$$[S \circ T]_{\mathcal{A}}^{\mathcal{C}} = [S]_{\mathcal{B}}^{\mathcal{C}} \cdot [T]_{\mathcal{A}}^{\mathcal{B}}$$

__Pf__ By homework.

## 3.14 Theorem: The Algebra of Matrices

__Thm__ $A,B,C,D,E$ are matrices with dimensions that make sense.

\begin{enumerate}
\item[(a)] $A(B+C) = AB + AC$. 
\item[(b)] $(D+E)A = DA + EA$.
\item[(c)] $a(AB) = (aA)B = A(aB), \forall a \in F$.
\item[(d)] $I_mA = A = AI_n,$ where 
$$I_j = \begin{bmatrix} 
1 & \dots & 0 \\
\vdots & \ddots & \vdots\\
0 & \dots & 1 \\
\end{bmatrix} .$$

\item[(e)] if dim$V$=n and $V$ has basis $\mathcal{B}$ then
$$[I_V]_{\mathcal{B}} = I_n$$
\item[(f)] $A(BC) = (AB)C$.

\end{enumerate}

## 3.15 Definition: Invertible Matrix

__Def__ We say $A \in M_{n \times n}(F)$ is invertible if $\exists B \in M_{n \times n}(F)$
such that $AB = BA = I_n$.

## 3.16 Theorem

__Thm__ If $A \in M_{n \times n}(F)$ is invertible then its inverse is unique, 
call it $A^{-1}$.

__Pf__ By Homework.

## 3.17 Theorem

__Thm__ Let $T: V \rightarrow W$ be linear, $\mathcal{B}$ a basis for $V$, $\mathcal{C}$ a basis for $W$.
Finally, suppose $V,W$ finite dimensional.
Then:
\begin{enumerate}
\item $T$ invertible $\iff [T]_\mathcal{B}^\mathcal{C}$ is invertible.
\item $[T^{-1}]_\mathcal{B}^\mathcal{C} = ([T]_\mathcal{B}^\mathcal{C})^{-1}$
	
\end{enumerate}

__Pf of (i)__

Assume $T$ is invertible. $T$ is invertible $\iff$ $T$ is an $F$-isomorphism,
by 2.21. $V \cong W \cong F^n$, where the dim$V$=n.
So $[T]_\mathcal{B}^\mathcal{C}$ is an $n\times n$ matrix.
Compute:
$$I_n = [I_V]_\mathcal{B}^\mathcal{B}, \text{ by 3.14(e) }$$
$$= [T^{-1} \circ T]_\mathcal{B}^\mathcal{B}$$
$$=[T^{-1}]_\mathcal{C}^\mathcal{B} [T]_\mathcal{B}^\mathcal{C}$$
Similarly
$$I_n = [I_W]_\mathcal{C}^\mathcal{C} = [T \circ T^{-1}]_\mathcal{C}^\mathcal{C} = [T]_\mathcal{B}^\mathcal{C}$$

Assume that $[T]_\mathcal{B}^\mathcal{C} = A$ is inv.
Let $B$ be such that $AB=BA = I_n$ and suppose $B=[\vec{b}_1 \ldots \vec{b}_n].$

Define $C = [\vec{v}_1 \ldots \vec{v}_n]$where $\mathcal{B}=\{\vec{v}_1, \ldots, \vec{v}_n \}$.
Also let $\mathcal{C} = \{\vec{w}_1, \ldots, \vec{w}_n \}$.

__Claim 1__: $\{C\vec{b}_1, \dots, C\vec{b}_n \}$ is a basis for $V$. 

__Pf__ By Homework.

*mapping a basis through an isomorphism is also a basis...check.*

Now, by 2.14, there exists $S:W \rightarrow V$ such that 

$$S(\vec{w}_j) = C\vec{b}_j.$$

__Claim 2__: $[S]_\mathcal{C}^\mathcal{B} = B$.

__Pf__ 
$$[S]_\mathcal{C}^\mathcal{B} = [[S(\vec{w}_1)]_\mathcal{B} \ldots [S(\vec{w}_1)]_\mathcal{B}], \text{ by defintion}$$
$$ = [[C\vec{b}_1]_\mathcal{B} \ldots [C\vec{b}_n]_\mathcal{B}]]$$
$$= [\vec{b}_1 \ldots \vec{b}_n] = B.$$

Finally,

$$[S \circ T]_\mathcal{B}^\mathcal{B} = [S]_\mathcal{C}^\mathcal{B} [T]_\mathcal{B}^\mathcal{C} = BA = [I_V]^\mathcal{B}_\mathcal{B}$$
$$\implies S \circ T = I_V.$$

__Pf of (ii)__

*9-22-22*

Recall from (i) that we constructed $S : W \to V$ such that
$$ S \circ T = I_V$$
$$T \circ S = I_W$$
which implies that $S=T^{-1}$.

Consider:

$$[I_V]_\mathcal{B}^\mathcal{B} = [S \circ T]_\mathcal{B}^\mathcal{B} = [S]_\mathcal{C}^\mathcal{B} [T]_\mathcal{C}^\mathcal{B} = I_n \text{ by 3.14(d) }$$

$$=[T^{-1}]_\mathcal{C}^\mathcal{B} [T]_\mathcal{C}^\mathcal{B} = I_n$$
$$=[T^{-1}]_\mathcal{C}^\mathcal{B} A^{-1} = I_n$$

Similarly, you can check the other side for $I_m$ as well.
So $A^{-1} = [T^{-1}]_\mathcal{C}^\mathcal{B}$ as desired.

## 3.18 Theorem

__Thm__ Let $V$ be an $n$-dimensional vectors $F$-v.s. with bases $\mathcal{B}$ 
and $\mathcal{C}$.

Let $Q = [I_V])_\mathcal{C}^\mathcal{B}$. Then:
\begin{enumerate}
\item[(i)] $Q$ is inv.
\item[(ii)] $\forall \vec{v} \in V,  Q[\vec{v}]_\mathcal{C} = [\vec{v}]_\mathcal{B}$.

So $Q$ is a change-of-basis matrix from $\mathcal{C}$-coordinates to $\mathcal{B}$-coordinates.
\end{enumerate}

__Pf__
__(i)__ Since $I_V : V \to W$ is inv., so is $[I_V]_\mathcal{C}^\mathcal{B}$ by 3.17(i).

__(ii)__ 
$$[\vec{v}]_\mathcal{B} =  [I_V(\vec{v})]_\mathcal{B} = [I_V]_\mathcal{C}^\mathcal{B} [\vec{v}]_\mathcal{C} = Q[\vec{v}]_\mathcal{C}.$$

## 3.19 Theorem

__Thm__ Let $V$ be $n$-dim. with bases $\mathcal{B}$ and $\mathcal{C}$.

If $Q = [I_V])_\mathcal{C}^\mathcal{B},$ then given $T: V \to V$, 

$$[T]_\mathcal{C}^\mathcal{C} = Q^{-1}[T]_\mathcal{B}^\mathcal{B}$$

__Pf__

Note that:
$$T = I_V \circ T = T \circ I_V$$
$$[T]_\mathcal{C}^\mathcal{B} = [I_V \circ T]_\mathcal{C}^\mathcal{B} = [T \circ I_V]_\mathcal{C}^\mathcal{B}.$$
Then:

$$[I_V \circ T]_\mathcal{C}^\mathcal{B} = [I_V]_\mathcal{C}^\mathcal{B} [T]_\mathcal{C}^\mathcal{C}$$

Similarly,

$$[T \circ I_V]_\mathcal{C}^\mathcal{B} = [T]_\mathcal{B}^\mathcal{B} [I_V]_\mathcal{C}^\mathcal{B}$$

So:

$$[I_V]_\mathcal{C}^\mathcal{B} [T]_\mathcal{C}^\mathcal{C} = [T]_\mathcal{B}^\mathcal{B} [I_V]_\mathcal{C}^\mathcal{B}$$

$$Q[T]_\mathcal{C}^\mathcal{C} = [T]_\mathcal{B}^\mathcal{B}Q$$

Thus:

$$[T]_\mathcal{C}^\mathcal{C} = Q^{-1} [T]_\mathcal{B}^\mathcal{B}Q$$

## 3.20 Definition: Change of Basis Matrix

__Def__ Let $A,B \in M_{mn}(F)$ then we say $B$ is *similar* to $A$ if

$$B = Q^{-1}AQ, \text{ for some inv. $Q \in M_{mn}(F)$}$$

ie, $B,A$ represent the same transformation with respect to different bases.


# 4 Computing with Matrices

## 4.1 Idea

Given some $T : V \to W$ basis $\mathcal{B}$ and $\mathcal{C}$, we have a matrix

$$A = [T]_\mathcal{B}^\mathcal{C} : \mathbb{R}^n \to \mathbb{R}^m.$$

Now we'll compute with $A$ instead of $T$.

## 4.2 Theorem

__Thm__ $V,W$ f.d v.s, and $T: V \rightarrow W$ an isomorphism.

\begin{enumerate}
\item If $U$ is a subspace of V, then $T(U)$ is subspace of $W$.
\item dim$(T(U)) = $dim$(U)$.
\end{enumerate}

__Pf__ By Homework.

## 4.3 Theorem

__Thm__ $V,W$ f.d v.s, and $T: V \rightarrow W$ an isomorphism. The following are true

\begin{enumerate}
\item[(i)] $\{\vec{v}_1,\ldots, \vec{v}_s \}$ is lin. ind. in $V \iff \{T(\vec{v}_1, \ldots, T(\vec{v}_s)\}$.
\item[(ii)]$\{\vec{v}_1,\ldots, \vec{v}_s \}$ spans $V \iff \{T(\vec{v}_1, \ldots, T(\vec{v}_s)\}$ spans W.
\end{enumerate}


## 4.4 Theorem

__Thm__ $V,W$ f.d v.s, with basis $\mathcal{B}$ and $\mathcal{C}$ respectively 
\begin{enumerate}
\item[(i)] Given $T: V \to W$, 
rank$(T)$ = rank$([T]_\mathcal{B}^\mathcal{C})$
\item[(ii)] nullity$(T)$ = nullity$([T]_\mathcal{B}^\mathcal{C})$.
\end{enumerate}


__Pf__ By rank-nullity, $n = \text{rank}(T) + \text{nullity}(T)$
and $n=\text{rank}([T]_\mathcal{B}^\mathcal{C}) + \text{nullity}([T]_\mathcal{B}^\mathcal{C})$.

Want to show that 
$$\text{nullity}(T) = \text{nullity}([T]_\mathcal{B}^\mathcal{C}).$$

By 4.2, the image of $[\text{Ker}(T)]_\mathcal{B}$ is a subspace of $\mathbb{R}^n$
and it has the same dimension of the kernel of $T$.

$$\text{nullity}(T) = \text{dim(Ker}(T)) = \text{dim([Ker}(T)]_\mathcal{B}).$$

Claim: $[\text{Ker}(T)]_\mathcal{B} = \text{Ker}([T]_\mathcal{B}^\mathcal{C})$.

*This is actually overkill, but we get something stronger!*

Note: 

$$[\text{Ker}(T)]_\mathcal{B} = \{ [\vec{v}]_\mathcal{B} \mid \vec{v} \in V \mid T(\vec{v}) = \vec{0} \}$$

$$\text{Ker}([T]_\mathcal{B}^\mathcal{C}) = \{ \vec{x} \in \mathbb{R}^n \mid [T]_\mathcal{B}^\mathcal{C} \vec{x}= \vec{0} \}$$
$$= \{ \vec{x} \in \mathbb{R}^n \mid [T]_\mathcal{B}^\mathcal{C}[\vec{v}]_\mathcal{B} = \vec{0}, \vec{x} = [\vec{v}]_\mathcal{B} \}$$
This is because $[\cdot]_\mathcal{B}$ is an isomorphism.

$$= \{ [\vec{v}]_\mathcal{B} \in \mathbb{R}^n \mid [T]_\mathcal{B}^\mathcal{C}[\vec{v}]_\mathcal{B} = \vec{0}, \vec{v}\in V \} $$
$$= \{ [\vec{v}]_\mathcal{B} \mid [T(\vec{v})]_\mathcal{C} = \vec{0}, \vec{v} \in V \}$$
$$ \{ [\vec{v}]_\mathcal{B} \mid [T(\vec{v})]_\mathcal{C} = [\vec{0}]_\mathcal{C}, \vec{v}\in V\}$$
This is because $[\cdot]_\mathcal{C}$ is an isomorphism.

$$= \{ [\vec{v}]_\mathcal{B} \mid T(\vec{v}) = \vec{0}, \vec{v} \in V \},$$ 
as desired.


## 4.5 Theorem

__Thm__ Let $S = \{ \vec{v}_1, \dots, \vec{v}_n \} \subseteq \mathbb{R}^m$, and 
$A = [\vec{v}_1 \ldots \vec{v}_n],$ so A is $m\times n$.

Then
\begin{enumerate}
\item[(i)] $S$ is lin. ind. if and only if $\text{rank}(A) = n$.
\item[(ii)] $S$ spans $R^m$ if and only if $\text{rank}(A) = m$.
\item[(iii)] $S$ is a basis for $R^m$ if and only if $\text{rank}(A) = m = n.$
\end{enumerate}

__Pf__ By Homework.

## 4.6 Example

We can use Theorems 4.3 and 4.5 to transform problems into statements about 
matrices:

Question: Is $S = \{ x^2 + 2x -1, 4x + 1, x^2 + 7\} \subset P_2(\mathbb{R})$ linearly independent?

Choose $\mathcal{B} = \{ x^2, x, 1\}$ as a basis. Then $S$ is linearly independent $\iff [S]_\mathcal{B}$ 
is lin. ind. by Theorem 4.3. By 4.5 $S$ is lin. ind. if and only if the matrix below, $A$,
has rank 3.

$$
\begin{bmatrix} 
1	&	0	&	1 \\
2	&	4	&	0 \\
-1	&	1	&	7 \\
\end{bmatrix} .
$$

Now, how do we determine the rank of a matrix?

## 4.7 Theorem

__Thm__ Let $A_{mn}, P_{mn}, Q_{mn}$ be matrices with $P,Q$ invertible.
Then:
\begin{enumerate}
\item[(a)] rank$(AQ) = \text{rank(A)}$.
\item[(b)] rank$(PA) = \text{rank(A)}$.
\item[(c)] rank$(PAQ) = \text{rank(A)}$.	
\end{enumerate}

__Pf__ By Homework.


## 4.8 Definition: Elementary Row Operations (ERO)

\begin{enumerate}
\item[(i)] Row Interchange.
$$
\begin{bmatrix} 
1	&	2	&	3	\\
4	&	5	&	6	\\
7	&	8	&	9	\\
\end{bmatrix} 
\sim
\begin{bmatrix} 
4	&	5	&	6	\\
1	&	2	&	3	\\
7	&	8	&	9	\\
\end{bmatrix} 
$$
$$R_1 \leftrightarrow R_2$$

\item[(ii)] Row Scaling (non-zero!)
$$
\begin{bmatrix} 
1	&	2	&	3	\\
4	&	5	&	6	\\
7	&	8	&	9	\\
\end{bmatrix} 
\sim
\begin{bmatrix} 
1	&	2	&	3	\\
8	&	10	&	11	\\
7	&	8	&	9	\\
\end{bmatrix} 
$$
$$2R_2 \rightarrow R_2$$

\item[(iii)] Row Replacement
$$
\begin{bmatrix} 
1	&	2	&	3	\\
4	&	5	&	6	\\
7	&	8	&	9	\\
\end{bmatrix} 
\sim
\begin{bmatrix} 
1	&	2	&	3	\\
6	&	9	&	12	\\
7	&	8	&	9	\\
\end{bmatrix} 
$$
$$2R_1 + R_2 \rightarrow R_2$$
\end{enumerate}

## 4.9 Definition

__Def__ Let $I_n$ be the $n\times n$ identity. Perform a single ERO, say $R_*$,
on $I_n$ to get $E$:

$$I_n \underset{R_*}{\sim} E$$

We call $E$ an **elementary matrix**.

## 4.10 Example

$$ I_3 = \begin{bmatrix} 
1 & 0 & 0 \\
0 & 1 & 0 \\
0 & 0 & 1 \\
\end{bmatrix} 
\sim
\begin{bmatrix} 
1 & 0 & 0 \\
4 & 1 & 0 \\
0 & 0 & 1 \\
\end{bmatrix} 
= E
$$
$$4R_1  + R_2 \rightarrow R_2$$

Consider,

$$ A = \begin{bmatrix} 
a 	&	b	&	c	\\
d	&	e	&	f	\\
g	&	h	&	i	\\
\end{bmatrix} 
$$
Then
$$ EA = \begin{bmatrix} 
a 	&	b	&	c	\\
4a+d	&	4b+e	&	4c+f	\\
g	&	h	&	i	\\
\end{bmatrix} 
$$

## 4.11 Theorem

__Thm__ If $A_{m \times n}$ is a matrix and $E$ is an elementary matrix, then $EA$ is 
the matrix that results from performing $R_*$ on $A$, where $I \underset{R_*}{\sim} E$

__Pf__ True, but annoying. See Example 4.10.


## 4.12 Theorem

__Thm__ Elementary matrices are invertible.

__Pf__ Case by case. 

\begin{enumerate}
\item[(i)] Interchange. If 
$$I_n \underset{R_i \leftrightarrow R_j}{\sim} E,$$
and 
$$I_n \underset{R_i \leftrightarrow R_j}{\sim} E,$$
then 
$$EE = I_n.$$

\item[(ii)] Row Scaling. If 
$$I_n \underset{cR_i \rightarrow R_i}{\sim} E$$
and 
$$I_n \underset{\frac{1}{c}R_i \rightarrow R_i}{\sim} F,$$ 
then 
$$FE = I_n$$
$$EF = I_n.$$

\item[(iii)] If 
$$I_n \underset{cR_i + R_j \rightarrow R_j}{\sim} E$$
and 
$$I_n \underset{-cR_i + R_j \rightarrow R_j}{\sim} F$$ 
then 
$$EF=FE= I_n.$$
	
\end{enumerate}

## 4.13 Corollary

Elementary matrices preserve rank!

__Pf__ Use 4.7 and 4.12.

## 4.14 Example

Is $\{ x^2 + 2x -1, 4x + 1, x^2 + 7 \}$ linearly independent? 
Previously: $S$ is lin. ind. $\iff$ A has rank 3.

$$
A = \begin{bmatrix}
	1 & 0 & 1 \\
	2 & 4 & 0 \\
	-1 & 1 & 7 \\
\end{bmatrix}
\underset{-2R_1 + R_2 \rightarrow R_2,  R_1 + R_3 \rightarrow R_3}{\sim}
\begin{bmatrix}
	1 & 0 & 1 	\\
	0 & 4 & -2 	\\
	0 & 1 & 8 	\\
\end{bmatrix}
$$
$$
\underset{R_2 \leftrightarrow R_3}{\sim}
\begin{bmatrix}
	1 & 0 & 1 	\\
	0 & 1 & 8 	\\
	0 & 4 & -2 	\\
\end{bmatrix}
\underset{-4R_2 + R_3 \rightarrow R_3}{\sim}
\begin{bmatrix}
	1 & 0 & 1 	\\
	0 & 1 & 8 	\\
	0 & 0 & -34 	\\
\end{bmatrix}
\underset{\frac{1}{-34}R_3 \rightarrow R_3}{\sim}
\begin{bmatrix}
	1 & 0 & 1 	\\
	0 & 1 & 8 	\\
	0 & 0 & 1 	\\
\end{bmatrix}
= B.
$$
$$\text{rank}(A) = \text{rank}(B) = \text{dim(Span}	\{ \vec{b_1}, \vec{b_2}, \vec{b_3} \})$$
$$= 3, \text{ since columns of $B$ span } \mathbb{R}^3.$$

## 4.15 Defintion

__Def__ A matrix is said to be in **Reduced Row Echelon Form** if 

\begin{enumerate}
\item[(i)] Rows with all 0's are below all rows that have non-zero entries.
\item[(ii)] The first non-zero entry in each row much be the only non-zero
entry in its column.
\item[(iii)] The first non-zero entry in each row must be 1, and this entry
must be to the right of the first non-zero entries above it.
\end{enumerate}

## 4.16 Examples

\begin{enumerate}
\item[(i)] RREF.
\begin{enumerate}
\item[(a)] 
$$
\begin{bmatrix}
1	 & 0  &2  \\
0	 & 1 &-1  \\
\end{bmatrix}
$$
\item[(b)]
$$
\begin{bmatrix}
	0 & 1 & 2 \\
	0 & 0 & 0 \\
	0 & 0 & 0 \\
\end{bmatrix}
$$
\item[(c)]
$$
\begin{bmatrix}
	1 & 2 & 0 \\
	0 & 0 & 1 \\
	0 & 0 & 0 \\
\end{bmatrix}
$$
\end{enumerate}
	
\item[(ii)] Not in RREF:
\begin{enumerate}
\item[(a)]
$$
\begin{bmatrix}
	1 & 2 \\
	0 & 1 \\
\end{bmatrix}
$$
\item[(b)]
$$
\begin{bmatrix}
	1 & 0 & 0 \\
	0 & 0 & 0 \\
	0 & 1 & 0 \\
\end{bmatrix}
$$

\item[(c)]
$$
\begin{bmatrix}
	1 & 0 \\
	0 & -1 \\
	0 & 0 \\
	0 & 0 \\
\end{bmatrix}
$$
\end{enumerate}
\end{enumerate}

## 4.17 Descriptive Example of Row Reduction

Idea: Use ERO's on a matrix to get it to RREF.

\begin{enumerate}
\item[(a)]
$$
\begin{bmatrix}
	1 & 4 & -5 & 0 \\
	2 & -1 & 8 & 9 \\
\end{bmatrix}
\underset{-2R_1 + R_2 \to R_2}{\sim}
\begin{bmatrix}
	1 & 4 & -5 & 0 \\
	0 & -9 & 18 & 9 \\
\end{bmatrix}
$$
$$
\underset{-\frac{1}{9} R_2 \to R_2}{\sim}
\begin{bmatrix}
	1 & 4 & -5 & 0 \\
	0 & 1 & -2 & -1 \\
\end{bmatrix}
\underset{-4R_2 \to R_1}{\sim}
\begin{bmatrix}
	1 & 0 & -3 &  4 \\
	0 & 1 & -2 & -1 \\
\end{bmatrix}
.
$$
\item[(b)]
$$
\begin{bmatrix}
	0 & 1 & 4 \\
	1 & 2 & -1 \\
	5 & 8 & 0 \\
\end{bmatrix}
\underset{R \leftrightarrow R_2}{\sim}
\begin{bmatrix}
	1 & 2 & -1 \\
	0 & 1 & 4 \\
	5 & 8 & 0 \\
\end{bmatrix}
\underset{-5R_1+R_3\to R_3}{\sim}
$$
$$
\begin{bmatrix}
	1 & 2 & -1 \\
	0 & 1 & 4 \\
	0 & -2 & 5 \\
\end{bmatrix}
\underset{2R_2 + R_3 \rightarrow R_3}{\sim}
\begin{bmatrix}
	1 & 2 & -1 \\
	0 & 1 & 4 \\
	0 & 0 & 13 \\
\end{bmatrix}
$$
$$
\underset{\frac{1}{13} R_3 \rightarrow R_3}{\sim}
\begin{bmatrix}
	1 & 2 & -1 \\
	0 & 1 & 4 \\
	0 & 0 & 1 \\
\end{bmatrix}
\underset{-4R_3 + R_2 \rightarrow R_2}{\sim}
\begin{bmatrix}
	1 & 2 & -1 \\
	0 & 1 & 0 \\
	0 & 0 & 1 \\
\end{bmatrix}
$$
$$
\underset{1R_3 + R_1 \rightarrow R_1}{\sim}
\begin{bmatrix}
	1 & 2 & 0 \\
	0 & 1 & 0 \\
	0 & 0 & 1 \\
\end{bmatrix}
\underset{-2R_2 + R_1 \rightarrow R_1}{\sim}
\begin{bmatrix}
	1 & 0 & 0 \\
	0 & 1 & 0 \\
	0 & 0 & 1 \\
\end{bmatrix}
$$
\end{enumerate}

## 4.18 Definition

__Def__ We say two matrices a **row equivalent** if you can get from one to the
other using elementary row operations. We write $A \sim B$ is A is row equivalent 
to $B$.  

$$ A \sim B \iff \exists E_1, \ldots , E_k \text{ such that } E_k \cdots E_1A = B$$

where $E_1,\ldots, E_k$ are elementary matrices.

## 4.19 Theorem

__Thm__ Every matrix is row equivalent to a matrix in RREF. Moreover, this RREF
is unique.

$$\forall A, \exists! B \text{ such that } A \sim B \text{ and $B$ is in RREF. }$$

__Pf__ Tedious, omitted.

## 4.20 Theorem

__Thm__ If $A,B$ are invertible matrices, then $AB$ is invertible and 
$$(AB)^{-1} = B^{-1}A^{-1}.$$

__Pf__ Check

$$(AB)(B^{-1}A^{-1}) = I.$$
$$(B^{-1}A^{-1})(AB) = I.$$

## 4.21 Theorem

__Thm__ Let $A$ be $n\times n$. The following are equivalent ( TFAE ):
\begin{enumerate}
\item[(i)] rank$(A) = n$.
\item[(ii)] $A$ is invertible.
\item[(iii)] $A \sim I_n$.
\end{enumerate}

__Pf__ H.W. Go $(iii) \to (ii) \to (i) \to (iii)$

## 4.22 Theorem

__Thm__ Let $A_{n \times n}$ and $A$ invertible, then if $E_1, \ldots, E_k$ are elementary
matrices such that 
$$E_k \cdots E_1 A = I_n$$ 
then
$$E_k \cdots E_1 I_n = A^{-1}$$

__Pf__ 
$$E_k \cdots E_1 A = I_n$$ 
Multiply by $A^{-1}$:
$$E_k \cdots E_1 I_n = A$$ 

## 4.23 Example

Find the inverse of 
$$A =
\begin{bmatrix}
	0 & 1 & 2 \\
	1 & 0 & 3 \\
	4 & -3 & 8 \\
\end{bmatrix}
.
$$
First consider $[A\mid I_3]$.

$$
\begin{bmatrix}
	0 & 1 & 2 & 1 & 0 & 0 \\
	1 & 0 & 3 & 0 & 1 & 0 \\
	4 & -3 & 8 & 0 & 0 & 1 \\
\end{bmatrix}
\sim
\begin{bmatrix}
	1 & 0 & 3 & 0 & 1 & 0 \\
	0 & 1 & 2 & 1 & 0 & 0 \\
	4 & -3 & 8 & 0 & 0 & 1 \\
\end{bmatrix}
$$
$$
\sim
\begin{bmatrix}
	1 & 0 & 3 & 0 & 1 & 0 \\
	0 & 1 & 2 & 1 & 0 & 0 \\
	0 & -3 & -4 & 0 & -4 & 1 \\
\end{bmatrix}
\sim
\begin{bmatrix}
	1 & 0 & 0 & 2 & 7 & -\frac{3}{2} \\
	0 & 1 & 0 & -2 & 4 & -1 \\
	0 & 0 & 1 & \frac{3}{2} & -2 & \frac{1}{2} \\
\end{bmatrix}
$$

## Intermission for MatLab Training
*10-11-22*

## 4.24 Theorem: Inverse for a $2 \times 2$ Matrix

__Thm__ Let 

$$
A = 
\begin{bmatrix}
	a & b \\
	c & d \\
\end{bmatrix}
$$

Then $A$ is invertible if and only if $ad -bc \neq 0$. In this case
$$A^{-1} = \frac{1}{ad-bc}
\begin{bmatrix}
	d & -b \\
	-c & a \\
\end{bmatrix}
.
$$

__Pf__ If $ad-bc \neq 0$, then 
$$
\frac{1}{ad-bc}
\begin{bmatrix}
	d & -b \\
	-c & a \\
\end{bmatrix}
$$
exists, and is an inverse by inspection. So $A$ is invertible.

Now, assume $A$ is inv. By Theorem 4.21, $A \sim I$.

Let
$$A = 
\begin{bmatrix}
	a & b \\
	c & d \\
\end{bmatrix}
.$$

Now row reduce: 
$$
\begin{bmatrix}
	a & b \\
	c & d \\
\end{bmatrix}
\sim
\begin{bmatrix}
	a & b \\
	ac & ad \\
\end{bmatrix}
\sim
\begin{bmatrix}
	a & b \\
	0 & ad-bc \\
\end{bmatrix}
.$$

We know this must be row equivalent to the identity, so $ad-bc \neq 0$. 

## 4.25 Theorem: The Invertible Matrix Theorem

__Thm__ $A$ and $n \times n$ matrix. TFAE
\begin{enumerate}
\item[(a)] $A$ is invertible.
\item[(b)] $A : \mathbb{R}^n \to \mathbb{R}^n$ is an isomorphism. 
\item[(c)] The columns of $A$ span $\mathbb{R}^n$.
\item[(d)] The columns of $A$ are linearly independent. 
\item[(e)] $A\vec{x} = \vec{0} \iff \vec{x}= \vec{0}$.
\item[(f)] $A : \mathbb{R}^n \to \mathbb{R}^n$ is injective.
\item[(g)] $A : \mathbb{R}^n \to \mathbb{R}^n$ is surjective.
\item[(h)] $A \sim I_n$.
\item[(i)] Columns of $A$ are a basis for $\mathbb{R}^n$.
\item[(j)] Ker$(A) = 	\{ \vec{0} \}$.
\item[(k)] Range$(A) = \mathbb{R}^n$.
\item[(l)] rank$(A) = n$.
\end{enumerate}

__Pf__ Consequence of previous theorems.

## 4.26 Example of an Invertible $3 \times 3$

Consider 
$$
A= 
\begin{bmatrix}
	a & b & c \\
	d & e & f \\
	g & h & i \\
\end{bmatrix}
\sim 
\begin{bmatrix}
	a & b & c \\
	ad & ae & af \\
	ag & ah & ai \\
\end{bmatrix}
\sim	
\begin{bmatrix}
	a & b & c \\
	0 & ae-bd & af-cd \\
	0 & ah-bd & ai-cd \\
\end{bmatrix}
$$
$$ 
\sim	
\begin{bmatrix}
	a & b & c \\
	0 & ae-bd & af-cd \\
	0 & ah-bd & ai-cd \\
\end{bmatrix}
\sim
\begin{bmatrix}
	a & b & c \\
	0 & ae-bd & af-cd \\
	0 & (ah-bd)(ae-bd) & (ai-cd)(ae-bd) \\
\end{bmatrix}
$$
$$
\sim
\begin{bmatrix}
	a & b & c \\
	0 & ae-bd & af-cd \\
	0 & 0 & * \\
\end{bmatrix}
$$

where $* = (ai-cg)(ae-bd) -(ah-bg)(af-cd)$.

$$(ai-cg)(ae-bd) -(ah-bg)(af-cd) = a(aei -afh - bdi + bfg + cdh -ceg)$$
$$= \underbrace{a\left[ a(ei-fh) - b(di -fg) + c(dh -eg) \right]}_{detA}.$$
 So $A$ inv iff $det A \neq 0$.

*Note:* need $a \neq 0$ and one of $\{ ae -bd, ah-bg \}$ not zero.


## 4.27 Definition

__Def__ Let $A$ be $n \times n$. Define $A_{ij}$ by taking $A$ and removing the
$i$th row and $j$th column.

So $A_{ij}$is an $(n-1) \times (m-1)$ matrix.

e.g.
$$A = 
\begin{bmatrix}
	1 & 4 & -1 \\
	0 & \pi & 2 \\
	\sqrt{2} & 7 & \frac{3}{2} \\
\end{bmatrix}
,
$$
$$A_{23} = 
\begin{bmatrix}
	1 & 4 \\
	\sqrt{2} & 7 \\
\end{bmatrix}
.
$$

## 4.28 Definition

__Def__ Let $A$ be $n \times n$.
\begin{enumerate}
\item[(i)] If $n=2$, then define 
$$\text{det}A= a_{11}a_{22} - a_{12}a_{21}$$

\item[(ii)] If $n > 2$, then define 
$$\text{det}A = \sum_{j=1}^n (-1)^{1+j} a_{1j} \text{det}(A_{1j})$$
\end{enumerate}

## 4.29 Definition

__Def__ Let $A$ be $n \times n$. The $(i,j)$-cofactor of $A$ is 
$$C_{ij} = (-1)^{i+j} \text{det}(A_ij)$$


## 4.30 Theorem

__Thm__ Let $A$ be $n \times n$:

$$\text{det}(A) = \sum_{i=1}^n a_{ij}C_{ij}, \text{ for fixed j }$$
or
$$= \sum_{j=1}^n a_{ij}C_{ij}, \text{ for fixed i. }$$

__Pf__ Tedious, omitted.


## 4.31 Examples

\begin{enumerate}
\item[(a)]
$$A=
\begin{bmatrix}
-4 & \frac{3}{2} & \sqrt{2} \\
\pi	 & 7 & 3 \\
0	 & 0 & 1 \\
\end{bmatrix}
$$
Let's fix the 3rd row.

$$
\text{det}A = 1 \cdot 
\begin{vmatrix}
	-4 & \frac{3}{2} \\
	\pi & 7 \\
\end{vmatrix}
= -28 -\frac{3\pi}{2}.
$$
	
\item[(b)]
$$
B = 
\begin{bmatrix}
	1 & 0 & 0 & 0 & 0 \\
	-1 & 2 & 0 & 0 & 0 \\
	7 & 22.5 & 3 & 0 & 0 \\
\frac{3}{100} & \sqrt{5} & 2i-3 & 4 & 0 \\
	16.2 & \frac{1}{1000} & 34 & 0 & 5 \\
\end{bmatrix}
$$
$$
\text{det}(B) = 5 \cdot 
\begin{vmatrix}
	1 & 0 & 0 & 0 \\
	-1 & 2 & 0 & 0 \\
	7 & 22.5 & 3 & 0 \\
\frac{3}{1000} & \sqrt{5} & 2i-3 & 4 \\
\end{vmatrix}
$$
$$
= 5 \cdot 4 \cdot 
\begin{vmatrix}
	1 & 0 & 0 \\
	-1 & 2 & 0 \\
	7 & 22.5 & 3 \\
\end{vmatrix}
=
5 \cdot 4 \cdot 3 \cdot 2 \cdot 1
=
120
$$
\end{enumerate}



## 4.31 Theorem

__Thm__ Let $A,B$ be $n \times n$

\begin{enumerate}
\item[(i)] $A$ triangular $\implies$ det $A = \Pi_{i=1}^n a_{ii}$
\item[(ii)] $A \sim B$ by a row replacement $\implies$ det($A$) = det($B$).
\item[(iii)] $A \sim B$ by a row interchange $\implies$ det($B$) = -det$A$.
\item[(iv)] $A \underset{kR_i \to R_i}{\sim} B \to \text{det}B = k\text{det}A$
\item[(v)] $A$ is invertible $\iff \text{det}A \neq 0$
\item[(vi)] det$(AB) = $det$(A)$det$(B)$
\end{enumerate}

__Pf__ 

(i)-(iv), use cofactor expansion.

(v) $A$ inv $\iff A \sim I_n\iff A = E_p\ldots E_1I_n$
So det$A = (-1)^r k_1\ldots k_s,$ for $r \in \mathbb{N}, k_1 \ldots, k_s \in F.$

(vi) If $A$ or $B$ is not invertible, then both sides are 0. Otherwise, $A$ and $B$ are invertible, so write $A,B$ as product of elementary matrices and use previous results ((i)-(v)). 

# 5 Diagonalization

## 5.1 Motivation
*10-13-22*

\begin{enumerate}
\item[(a)] Suppose $\vec{x}_t \in \mathbb{R}^n$ represents the state of some system at time $t \in \mathbb{Z}$. Further, suppose $A: \mathbb{R}^n \to \mathbb{R}^n$ is linear with $\vec{x}_{t+1} = A\vec{x}_t$.

Given iniital state $\vec{x}_0 \in \mathbb{R}^n$:
$$\vec{x}_1 = A \vec{x}_0$$
$$\vec{x}_2 = A(A \vec{x}_0) = A^2 \vec{x}_0$$
$$\vdots$$
$$\vec{x}_{k+1} = A\vec{x}_k = \ldots = A^{k+1}\vec{x}_0$$

Suppose:

TODO: Diagram

$$A^2 = (PDP^{-1})^2$$
$$=PDP^{-1} PDP^{-1}$$
$$=PD^2P^{-1}$$

In general, $A^{k+1} = P^{k+1}P^{-1}$.

If $D=$diag$(d_1,...,d_n)$, then 
$$D^{k+1} = \text{diag}(d_1^{k+1}, \ldots, d_n^{k+1}).$$

\item[(b)] Let $T_{x=0}: \mathbb{R}^2 \to \mathbb{R}^2$ be the map that reflects points across the $y$-axis.

$$T\left(
\begin{bmatrix}
	x \\
	y \\
\end{bmatrix}
\right)
=
\begin{bmatrix}
	-x \\
	y \\
\end{bmatrix}
$$
If $\mathcal{S} = 	\{ e_1, e_2 \}$ be the std. basis for $\mathbb{R}^2$, then

$$
[T]_\mathcal{S}^\mathcal{S} = [[T(e_1)]_\mathcal{S}, [T(e_2)]_\mathcal{S}]
=
\begin{bmatrix}
	-1 & 0 \\
	0 & 1 \\
\end{bmatrix}
$$
Now, let $T_{y=2x}: \mathbb{R}^2 \to \mathbb{R}^2$ be the map that reflects points across $y=2x$.

Let's pick 
$$\begin{bmatrix} 
1 \\
2 \\
\end{bmatrix} 
\text{ and }
\begin{bmatrix} 
-2 \\
1 \\
\end{bmatrix} 
$$
Note:
$$
T\left(
\begin{bmatrix} 
1 \\
2 \\
\end{bmatrix} 
\right)
=
\begin{bmatrix} 
1 \\
2 \\
\end{bmatrix} 
\text{ and }
T\left(
\begin{bmatrix} 
-2 \\
1 \\
\end{bmatrix} 
\right)
=
\begin{bmatrix} 
2 \\
-1 \\
\end{bmatrix} 
$$
Now, 
$$
[T_{y=2x}]_\mathcal{B}^\mathcal{B} = 
\begin{bmatrix}
	1 & 0 \\
	0 & -1 \\
\end{bmatrix}
$$
Then
$$
[T(\vec{x})]_\mathcal{B} = [T_{y=2x}]_\mathcal{B}^\mathcal{B}[\vec{x}]_\mathcal{B}
$$
Problem: Given $\vec{x} \in \mathbb{R}^2$, how do we find $[\vec{x}]_\mathcal{B}$? Need a change-of-basis matrix:

$$
Q:= [I]_\mathcal{B}^\mathcal{S} =
\begin{bmatrix}
	1 & -2 \\
	2 & 1 \\
\end{bmatrix}
$$
Then
$$
Q^{-1} = [I]_\mathcal{S}^\mathcal{B} =
\frac{1}{5}
\begin{bmatrix}
	1 & 2 \\
	-2 & 1 \\
\end{bmatrix}
.
$$

So $[T]_\mathcal{B}^\mathcal{B} = Q^{-1}[T]_\mathcal{S}^\mathcal{S}Q$. Solve for A:
$$A=
\begin{bmatrix}
	1 & -2 \\
	2 & 1 \\
\end{bmatrix}
\begin{bmatrix}
	1 & 0 \\
	0 & -1 \\
\end{bmatrix}
\frac{1}{5}
\begin{bmatrix}
	1 & -2 \\
	2 & 1 \\
\end{bmatrix}
=
\frac{1}{5}
\begin{bmatrix}
	-3 & 4 \\
	4 & 3 \\
\end{bmatrix}
$$

So 
$$
T_{y=2z}\left(
\begin{bmatrix}
	x \\
	y \\
\end{bmatrix}
\right)
=
\frac{1}{5}
\begin{bmatrix}
	-3x+4y \\
	 4x+3y  \\
\end{bmatrix}
$$
TODO: Beautiful diagram.
\end{enumerate}		

## 5.2 Definition

__Def__ A linear map $T: V \to V$, $V$ f.d, is called __diagonalizable__ if there 
exists a basis $\mathcal{B}$ for $V$ such that $[T]_\mathcal{B}^\mathcal{B}$ is a diagonal matrix.

## 5.3 Example

If $\mathcal{B} = \{ \vec{v}_1, \ldots, \vec{v}_n \}$ is a basis for $V$ such 
that $T: V \to V$ has $[T]_\mathcal{B}^\mathcal{B}$ diagonal with
$$[T]_\mathcal{B}^\mathcal{B} = D = \text{diag}(d_1, \ldots, d_n)$$
then,
$$[T(\vec{v}_i)]_\mathcal{B} = [T]_\mathcal{B}^\mathcal{B} [\vec{v}_i]\mathcal{B}$$
$$=
\begin{bmatrix} 
d_1	&	& 0\\
	&	\ddots & \\
0	&	&	d_n\\
\end{bmatrix} 
\begin{bmatrix} 
0 \\
\vdots \\
1\\
\vdots\\
0
\end{bmatrix} 
=
d_i[\vec{v}_i]_\mathcal{B}
$$
