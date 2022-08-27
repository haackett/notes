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
7. $1 \times \vec{x} = \vec{x}$
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
