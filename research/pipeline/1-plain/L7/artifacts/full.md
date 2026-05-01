# L7: Portfolio Optimization and Performance Measures

Course Code: COMP7415

# Agenda

- Calculating risk and return of an investment portfolio   
Correlation effect for diversification   
- Minimum variance portfolio   
- Efficiency frontier   
- Tangency portfolio   
- Capital Asset Pricing Model (CAPM)

- Capital Market Line (CML)   
Security Market Line (SML)

Commonly used performance measures   
- Stop loss and risk limit

# Risk Management Cycle

1. Risk Identification   
2. Risk Assessment / Measurement   
3. Risk Treatment   
4. Risk Monitoring

![](images/79e05f423494a59b8f72adfb80a011ffb25c27e6f2d4fc60467d5150d464c33f.jpg)

# Investment Portfolio

# Investment Portfolio

- There are many different kinds of assets in the market (eg. stocks, commodity, forex, bonds, etc). We can pick various assets for our investment as a portfolio   
- Portfolio optimization is the process of selecting the best distribution of assets to maximize returns while minimizing risk   
- Investment basically involves

- asset selection   
- asset allocation

- In this section, we will discuss the Markowitz Portfolio Theory (or modern portfolio theory) which is a Nobel prize theory in 1990

# Revision of Basic Statistics

- Given that $a$ and $b$ are constants and $X$ and $Y$ are random variables.

$E(a + X) = a + E(X)$   
$\cdot E (a \times X) = a \times E (X)$   
$\cdot E(X + Y) = E(X) + E(Y)$   
- $Var(a + X) = Var(X)$   
$\operatorname{Var}(a \times X) = a^{2} \times \operatorname{Var}(X)$   
- $\operatorname{Var}\left( {X + Y}\right)  = \operatorname{Var}\left( X\right)  + \operatorname{Var}\left( Y\right)  + 2 \times  \operatorname{Cov}\left( {X,Y}\right)$   
$\operatorname{Cov}(a, X) = 0$   
$\operatorname{Cov}(a \times X, b \times Y) = a \times b \times \operatorname{Cov}(X, Y)$

# Portfolio Return and Risk

- Suppose a portfolio $P$ contains $n$ assets   
- Portfolio return will be

$$
R _ {p} = \sum_ {i = 1} ^ {n} w _ {i} R _ {i}
$$

where $\begin{array}{r}\left\{ \begin{array}{ll}w_{i} = \mathrm{weight~of~the~asset~in~the~portfolio}\\ \sum_{i = 1}^{n}w_{i} = 1 \end{array} \right. \end{array}$

Expected Portfolio Return

$$
E (R _ {p}) = \sum_ {i = 1} ^ {n} w _ {i} E (R _ {i})
$$

- Portfolio Variance

$$
\sigma_ {p} ^ {2} = \sum_ {i = 1} ^ {n} w _ {i} ^ {2} \sigma_ {i} ^ {2} + \sum_ {i = 1} ^ {n} \sum_ {i \neq j} w _ {i} w _ {j} \sigma_ {i j}
$$

where

$$
\begin{array}{l} \sigma_ {p} ^ {2} = \mathrm {v a r i a n c e} \\ \sigma_ {i} ^ {2} = \mathrm {v a r i a n c e} i \\ \sigma_ {i j} = \text {c o v a r i a n c e b e t w e e n a s s e t} I \text {a n d} j \\ \end{array}
$$

# Example

Given 2 risky assets, A and B   
• At time 0, \(P_{A,0} = \\)1\) and \(P_{B,0} = \$100\)   
- At time 1, you expect that the asset price may go up or down according to the economic conditions

<table><tr><td>Economic Condition</td><td>Probability</td><td>Asset Price A</td><td>Return of A</td><td>Asset Price B</td><td>Return of B</td></tr><tr><td>Bad</td><td>0.25</td><td>$0.6</td><td>(0.6-1)/1 = -40%</td><td>$70</td><td>(70-100)/100 = -30%</td></tr><tr><td>Normal</td><td>0.5</td><td>$1.0</td><td>(1-1)/1 = 0%</td><td>$110</td><td>(110-100)/100 = 10%</td></tr><tr><td>Good</td><td>0.25</td><td>$2.0</td><td>(2-1)/1 = 100%</td><td>$140</td><td>(140-100)/100 = 40%</td></tr></table>

• At time 0, you invest $40,000 in asset A and $60,000 in asset B (i.e. total investment $100,000). What is the expected return and variance of your portfolio?

# Example - Method 1

<table><tr><td>Economic Condition</td><td>Probability</td><td>Portfolio Value</td><td>Return of Portfolio</td></tr><tr><td>Bad</td><td>0.25</td><td>40000(1-40%) + 60000(1-30%) = 66000</td><td>66000/100000-1 = -34%</td></tr><tr><td>Normal</td><td>0.5</td><td>40000(1+0%) + 60000(1+10%) = 106000</td><td>106000/100000-1 = 6%</td></tr><tr><td>Good</td><td>0.25</td><td>40000(1+100%) + 60000(1+40%) = 164000</td><td>164000/100000-1 = 64%</td></tr></table>

- $\mu_{p} = 0.25\left(-34\%\right) + 0.5\left(6\%\right) + 0.25\left(64\%\right) = 10.5\%$   
- $\sigma_{P} = \sqrt{0.25(-34\% - 10.5\%)^{2} + 0.5(6\% - 10.5\%)^{2} + 0.25(64\% - 10.5\%)^{2}} = 34.94\%$

# Example - Method 2

$w_{A} = \frac{40000}{100000} = 0.4$   
$w_{B} = \frac{60000}{100000} = 0.6$   
- $\mu_{A} = E\left(R_{A}\right) = 0.25\left(-40\%\right) + 0.5\left(0\%\right) + 0.25\left(100\%\right) = 15 \%$   
- $\mu_{B} = E\left(R_{B}\right) = 0.25\left(-30\%\right) + 0.5\left(10\%\right) + 0.25\left(40\%\right) = 7.5 \%$   
- $\sigma_{A} = \sqrt{0.25(-40\% - 15\%)^{2} + 0.5(0\% - 15\%)^{2} + 0.25(100\% - 15\%)^{2}} = 0.5712$   
- $\sigma_{B} = \sqrt{0.25(-30\% - 7.5\%)^{2} + 0.5(10\% - 7.5\%)^{2} + 0.25(40\% - 7.5\%)^{2}} = 0.248747$   
- $\sigma_{AB} = 0.25(-40\% -15\%)(-30\% -7.5\%) + 0.5(0\% -15\%) (10\% -7.5\%) + 0.25(100\% -15\%) (40\% -7.5\%) = 0.11875$   
• $\rho_{AB} = \frac{0.11875}{0.5712 * 0.248747} = 0.92303$   
So, we have

$$
R _ {P} = w _ {A} R _ {A} + w _ {B} R _ {B}
$$

$$
\mu_ {P} = w _ {A} \mu_ {A} + w _ {B} \mu_ {B} = 0.4 * 15 \% + 0.6 * 7.5 \% = 10.5 \%
$$

$$
\sigma_ {P} = \sqrt {w _ {A} ^ {2} \sigma_ {A} ^ {2} + w _ {B} ^ {2} \sigma_ {B} ^ {2} + 2 w _ {A} w _ {B} \sigma_ {A B}} = 34.94 \%
$$

# 2 assets Portfolio

Using the previous example,   
- $\mu_{A} = 15 \%$   
- $\mu_{B} = 7.5\%$   
$\sigma_{A} = 0.5172$   
$\sigma_{B} = 0.248747$   
$\sigma_{AB} = 0.11875$   
$\rho_{AB} = 0.92303$

• For a general combination of A and B with weights $(w_{A}, w_{B})$ where $0 \leq w_{A}, w_{B} \leq 1$ and $w_{A} + w_{B} = 1$

![](images/802a07622e4579c5d0f8206bda9891f64260a907e1dd708d5c84764d52929d1d.jpg)  
Expected Return and Volatility of Portfolio (P) (with Correlation Coefficient $= 0.923026$

# 2 assets Portfolio - Correlation Effect

![](images/bbb5ee02bf44bdf91491bb59e88d29a2156fef48ec9df1640e3cda9d262bf887.jpg)

![](images/8b6b71e93b4395102003c45ff52787d5bf254e55efb411552e749b3d5875aa7a.jpg)

![](images/42187edb8faeae78f8a92fa7d6f81c608fe5584ffdf987ce2b9df9d32d374660.jpg)

![](images/307b6afaa851b44065e419ff530ed60d2dc60edcd843a5e884459cddd913c301.jpg)

![](images/f6d177c5275e5606329066d6f869b1c32aa2b9a93994c7cc3bf30ec400072e05.jpg)

# 2 assets Portfolio – Correlation Effect

- When the correlation is not very high, with appropriate weightings $(w_{A}$ and $w_{B})$ , the portfolio return can be higher than that of asset B but with a lower portfolio variance than that of asset B. As a risk averse investor, one would like to hold that portfolio rather than asset B alone.   
- The above diagrams illustrate that in most of the cases the overall risk of a portfolio of assets can be reduced but achieve the same expected return. This is called risk diversification   
- In general, if the 2 assets are more negatively correlated, it has better diversification effect

Expected Return (%)   
![](images/efe22f088163b82237160086bebcf5a00378a76d62a80ceda9e56344c93c8377.jpg)  
Standard Deviation $(\%)$

# Minimum Variance Portfolio - 2 assets

- Given

- $0 \leq w_{A}, w_{B} \leq 1$ and $w_{A} + w_{B} = 1$   
$R_{p} = w_{A} R_{A} + w_{B} R_{B}$   
$\mu_{p} = w_{A}\mu_{A} + w_{B}\mu_{B}$   
- $\sigma_{p}^{2}=w_{A}^{2} \sigma_{A}^{2}+w_{B}^{2} \sigma_{B}^{2}+2 w_{A} w_{B} \sigma_{A} \sigma_{B} \rho_{A B}$

- If $|\rho| \neq 1$ , let $w_A = w$ , $w_B = 1 - w$   
$\frac {\partial \mu_ {p}}{\partial w} = R _{A} - R _{B}$   
$\frac{\partial \sigma_{p}^{2}}{\partial w} = 2 w \left( \sigma_{A}^{2} + \sigma_{B}^{2} - 2 \rho \sigma_{A} \sigma_{B} \right) + 2 \sigma_{B} \left( \rho \sigma_{A} - \sigma_{B} \right)$   
$\frac{\partial^{2} \sigma_{p}^{2}}{\partial w^{2}} = 2 \left( \sigma_{A}^{2} + \sigma_{B}^{2} - 2 \rho \sigma_{A} \sigma_{B} \right) > 0$

So $\sigma_{p}^{2}$ has a global minimum at $w = \frac{-\sigma_{B}(\rho \sigma_{A} - \sigma_{B})}{\sigma_{A}^{2} + \sigma_{B}^{2} - 2\rho \sigma_{A} \sigma_{B}}$   
If $0 \leq w \leq 1$ , $\min(\sigma_p)$ exists iff $\rho \leq \min\left(\frac{\sigma_A}{\sigma_B}, \frac{\sigma_B}{\sigma_A}\right)$ .

$$
\mathrm {L e t} y = \mu_ {p}, x = \sigma_ {p}
$$

# Minimum Variance Portfolio - 2 assets

- If $\rho = 1$ ,

$\mu_{p} = w \mu_{A} + (1 - w) \mu_{B}$   
- $\sigma_{p}^{2} = w^{2}\sigma_{A}^{2} + (1 - w)^{2}\sigma_{B}^{2} + 2w(1 - w)\sigma_{A}\sigma_{B} = (w\sigma_{A} + (1 - w)\sigma_{B})^{2}$   
$\sigma_{p} = w \sigma_{A} + (1 - w) \sigma_{B}$

So we have

$\frac{\sigma_{p} - \sigma_{B}}{\sigma_{A} - \sigma_{B}} = \frac{\mu_{p} - \mu_{B}}{\mu_{A} - \mu_{B}}$

i.e. a straight line $\frac{x - \sigma_B}{\sigma_A - \sigma_B} = \frac{y - \mu_B}{\mu_A - \mu_B}$

# Minimum Variance Portfolio - 2 assets

- If $\rho = -1$ ,

$$
\cdot \sigma_ {p} ^ {2} = (w \sigma_ {A} - (1 - w) \sigma_ {B}) ^ {2}
$$

- So

$$
\sigma_ {p} = \left\{ \begin{array}{l l} (1 - w) \sigma_ {B} - w \sigma_ {A} & \mathrm {I f} w <   \frac {\sigma_ {B}}{\sigma_ {A} + \sigma_ {B}} \\ 0 & \mathrm {I f} w = \frac {\sigma_ {B}}{\sigma_ {A} + \sigma_ {B}} \\ w \sigma_ {A} - (1 - w) \sigma_ {B} & \mathrm {I f} w > \frac {\sigma_ {B}}{\sigma_ {A} + \sigma_ {B}} \end{array} \right.
$$

# 2 assets Portfolio (with short selling allowed)

- For a general combination between asset A and asset B with weightings $(w_{A}, w_{B})$ ,   
- No restriction on $w_{A}, w_{B}$ , except that $w_{A} + w_{B} = 1$

# Example:

• Given that at time 0, \(P_{A,0} = \\)1\( and \(P_{B,0} = \\(100\), and originally you have \(\$ 100\) cash.   
• To create a portfolio with $w_{A} = 200\%$ and $w_{B} = -100\%$ , you need to:

- buy 200 shares of asset A (cash outflow $200); and   
- shortsell 1 share of asset B (cash inflow $100)

- The total initial investment cost of the portfolio is $100.

![](images/c2c0bd56213b8fb45da8009c25b9bcf91160faf6850659264f948a96c2b8e377.jpg)  
Expected Return and Volatility of Portfolio (P) (with Correlation Coefficient $= -0.5$

# Efficient Frontier

# Given a set of risky assets,

- Opportunity Set: The set of all possible portfolios.   
- Minimum Variance Frontier (MVF): the set of portfolios with the lowest variance for a given expected return.   
- Minimum Variance Portfolio (MVP) (or global minimum variance portfolio): The portfolio with the lowest standard deviation among all possible portfolios.   
- Efficient Frontier (or efficient set): Portfolios yielding the highest expected return for a specific variance, represented as the upper curve of the MVF.   
- Inefficient Portfolio: Portfolios not on the efficient frontier, which should be avoided as better options exist for the same or lower risk

# Efficient Frontier

- For 2-asset portfolio

![](images/aac61be223da3ccc5ee88169a8e6bdea2cded5b3e83c82e7ef5bb0dd3fae4243.jpg)  
Expected Return and Volatility of Portfolio (P) (with Correlation Coefficient $= -0.5$ )

Risk and Return of Portfolio P

100% in Asset A

100% in Asset B

Shortselling Asset A

Shortselling Asset B

# Efficient Frontier

- For n-asset portfolio

E(Rp)

![](images/1409b733918e2f6ac6e3e6a0241bc19370a3c1ad992dc2e345568026c41bc7ec.jpg)

# Solving for Global Minimum Variance Portfolio for N assets

# Global Minimum Variance Portfolio

# Given

$\sum_{i=1}^{N} w_{i} = 1$   
$R_{p} = \sum_{i=1}^{N} w_{i} R_{i}$   
$\mu_{p} = E\left(R_{p}\right) = \sum_{i=1}^{N} w_{i} R_{i}$   
$\sigma_{p}^{2} = \sum_{i=1}^{N} w_{i}^{2} \sigma_{i}^{2} + 2 \sum_{i<j} w_{i} w_{j} \sigma_{i} \sigma_{j} \rho_{ij}$

# Denote:

$$
\begin{array}{l} \cdot \Sigma = \left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) = \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right) \left( \begin{array}{c c c c} 1 & \rho_ {1 2} & \dots & \rho_ {1 N} \\ \rho_ {2 1} & 1 & \dots & \rho_ {2 N} \\ \vdots & \vdots & \ddots & \vdots \\ \rho_ {N 1} & \rho_ {N 2} & \dots & 1 \end{array} \right) \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right) \\ \cdot \underline {{\boldsymbol {w}}} = (w _ {1}, \dots , w _ {N}) ^ {T} \\ \cdot \underline {{\underline {{\mu}}}} = (\mu_ {1}, \dots , \mu_ {N}) ^ {T} \\ \end{array}
$$

# - So

$$
\begin{array}{l} \cdot \mu_ {p} = \underline {{w}} ^ {T} \underline {{\mu}} \\ \cdot \sigma_ {p} ^ {2} = \underline {{\boldsymbol {w}}} ^ {T} \Sigma \underline {{\boldsymbol {w}}} \\ \end{array}
$$

# Global Minimum Variance Portfolio

# Objective function:

• Min $\frac{1}{2} Var\left(R_{p}\right) = \frac{1}{2} \sum_{i=1}^{N} \sum_{j=1}^{N} w_{i} w_{j} \sigma_{ij}$

• Subject to   
$\sum_{i=1}^{N} w_{i} = 1$   
- Let $L = \frac{1}{2}\sum_{i=1}^{N}\sum_{j=1}^{N}w_{i}w_{j}\sigma_{ij} - \lambda\left(\sum_{i=1}^{N}w_{i} - 1\right)$   
- Then, $\frac{\partial L}{\partial w_k} = \sum_{i=1}^N w_i \sigma_{ik} - \lambda = 0 \quad \forall k = 1, \dots, N$   
So we have

$$
\left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) \left( \begin{array}{c} w _ {1} \\ \vdots \\ w _ {N} \end{array} \right) = \lambda \left( \begin{array}{c} 1 \\ \vdots \\ 1 \end{array} \right)
$$

or

$$
\Sigma_ {\underline {{W}}} = \lambda_ {\underline {{I}} _ {N}}
$$

# Global Minimum Variance Portfolio

- Then

$$
\cdot \underline {{W}} = \lambda \Sigma^ {- 1} \underline {{I _ {N}}}
$$

As $\sum_{i=1}^{N} w_{i} = 1$ , we get

$$
1 = \underline {{I _ {N}}} ^ {T} \underline {{w}} = \lambda \underline {{I _ {N}}} ^ {T} \Sigma^ {- 1} \underline {{I _ {N}}}
$$

$$
\lambda = (\underline {{I _ {N}}} ^ {T} \Sigma^ {- 1} \underline {{I _ {N}}}) ^ {- 1}
$$

Thus,

$$
\underline {{\boldsymbol {w}}} = \frac {1}{\underline {{\boldsymbol {I _ {N}}}} ^ {T} \Sigma^ {- 1} \underline {{\boldsymbol {I _ {N}}}}} \Sigma^ {- 1} \underline {{\boldsymbol {I _ {N}}}}
$$

# Deriving Efficient Frontier for N assets

# Deriving Efficient Frontier for N assets

- The idea is simply to iterate for each target return, then find the corresponding minimum variance portfolio

![](images/926a01df3275ffa0d2dc58f5907e54291c1d952f9d549da9ccdbfedc45eda4ee.jpg)

# Optimal Portfolio with a target return

# Given

$\sum_{i=1}^{N} w_{i} = 1$   
$R_{p} = \sum_{i=1}^{N} w_{i} R_{i}$   
$\mu_{p} = E\left(R_{p}\right) = \sum_{i=1}^{N} w_{i} R_{i} = \mu_{0}$   
$\sigma_{p}^{2} = \sum_{i=1}^{N} w_{i}^{2} \sigma_{i}^{2} + 2 \sum_{i<j} w_{i} w_{j} \sigma_{i} \sigma_{j} \rho_{ij}$

# Denote:

$$
\cdot \Sigma = \left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) = \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right) \left( \begin{array}{c c c c} 1 & \rho_ {1 2} & \dots & \rho_ {1 N} \\ \rho_ {2 1} & 1 & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ \rho_ {N 1} & \rho_ {N 2} & \dots & 1 \end{array} \right) \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right)
$$

$\underline{\underline{W}} = (w_{1},\dots ,w_{N})^{T}$

$\underline{\mu} = (\mu_{1},\dots,\mu_{N})^{T}$

# - So

$\mu_{p} = \underline{w}^{T}\underline{\mu} = \mu_{0}$   
$\sigma_{p}^{2} = \underline{w}^{T}\Sigma \underline{w}$

# Optimal Portfolio with a target return

Objective function:

$$
\cdot \operatorname {M i n} \frac {1}{2} \operatorname {V a r} \left(R _ {p}\right) = \frac {1}{2} \sum_ {i = 1} ^ {N} \sum_ {j = 1} ^ {N} w _ {i} w _ {j} \sigma_ {i j}
$$

• Subject to

$$
\begin{array}{l} \bullet \mu_ {p} = \sum_ {i = 1} ^ {N} w _ {i} \mu_ {i} = \mu_ {0} \\ \cdot \sum_ {i = 1} ^ {N} w _ {i} = 1 \\ \end{array}
$$

$\cdot$ Let $L = \frac{1}{2}\sum_{i=1}^{N}\sum_{j=1}^{N}w_{i}w_{j}\sigma_{ij} - \lambda_{1}\left(\sum_{i=1}^{N}w_{i}\mu_{i} - \mu_{p}\right) - \lambda_{2}\left(\sum_{i=1}^{N}w_{i} - 1\right)$   
$\frac{\partial L}{\partial w_{k}} = \sum_{i=1}^{N} w_{i} \sigma_{ik} - \lambda_{1} \mu_{k} - \lambda_{2} = 0 \quad \forall k = 1, \ldots, N$   
So we have

$$
\left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) \left( \begin{array}{c} w _ {1} \\ \vdots \\ w _ {N} \end{array} \right) = \left( \begin{array}{c c} \mu_ {1} & 1 \\ \vdots & \vdots \\ \mu_ {N} & 1 \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \qquad \mathrm {o r} \qquad \Sigma_ {\underline {{W}}} = \left(\underline {{\mu}} \underline {{I _ {N}}}\right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right)
$$

# Optimal Portfolio with a target return

- That is

$$
\begin{array}{l} \cdot \underline {{w}} = \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & I _ {N} \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \dots \dots \dots ( ^ {*}) \\ \bullet \mu_ {p} = \sum_ {i = 1} ^ {N} w _ {i} \mu_ {i} = \mu_ {0} \\ \cdot \sum_ {i = 1} ^ {N} w _ {i} = 1 \\ \end{array}
$$

Now we will find $\lambda_{1}, \lambda_{2}$ such that

$$
\begin{array}{l} \cdot \quad 1 = \underline {{I _ {N}}} ^ {T} \underline {{w}} = \underline {{I _ {N}}} ^ {T} \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & \underline {{I _ {N}}} \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \\ \cdot \mu_ {0} = \underline {{\mu}} ^ {T} \underline {{w}} = \underline {{\mu}} ^ {T} \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & \underline {{I _ {N}}} \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \\ \end{array}
$$

![](images/554a4dab82504ce3faad016b15627d934bfdb06b2c49cdb635254eff165bcc19.jpg)

$$
\left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) = \left(\left( \begin{array}{c} I _ {N} ^ {T} \\ \underline {{\mu}} ^ {T} \end{array} \right) \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & I _ {N} \end{array} \right)\right) ^ {- 1} \left( \begin{array}{c} 1 \\ \mu_ {0} \end{array} \right)
$$

- Substitute back to equation $(^{*})$ , we have

$$
\underline {{\boldsymbol {w}}} = \boldsymbol {\Sigma} ^ {- 1} \left( \begin{array}{l l} \underline {{\boldsymbol {\mu}}} & \underline {{\boldsymbol {I _ {N}}}} \end{array} \right) \left(\left( \begin{array}{l} \underline {{\boldsymbol {I _ {N}}}} ^ {T} \\ \underline {{\boldsymbol {\mu}}} ^ {T} \end{array} \right) \boldsymbol {\Sigma} ^ {- 1} \left( \begin{array}{l l} \underline {{\boldsymbol {\mu}}} & \underline {{\boldsymbol {I _ {N}}}} \end{array} \right)\right) ^ {- 1} \left( \begin{array}{l} 1 \\ \boldsymbol {\mu_ {0}} \end{array} \right)
$$

# Tangency Portfolio

# Tangency Portfolio

- If risk-free asset is available, we can draw a tangent line between the risk-free rate and the efficient frontier   
- The toughing point is called tangency portfolio   
- We can create any portfolios along the tangent line by proper allocation on risk-free asset and the tangency portfolio

![](images/854d8af52585526e0e5395fae93774bfa7c87dcd9cad02e9ef2bac845d8fd37d.jpg)

# Deriving Tangency Portfolio

In order to find the tangency portfolio, we consider to maximize the slope

$$
\max _ {w} \frac {\mu_ {p} - R _ {f}}{\sigma_ {p}}
$$

subject to $\sum_{i=1}^{N} w_{i} = 1$

- Consider the Lagrangian

$$
L = \frac {\mu_ {p} - R _ {f}}{\sigma_ {p}} - \lambda \left(\sum_ {i = 1} ^ {N} w _ {i} - 1\right)
$$

$$
\frac {\partial L}{\partial w _ {k}} = \frac {\sigma_ {p} \frac {\partial (\mu_ {p} - R _ {f})}{\partial w _ {k}} - (\mu_ {p} - R _ {f}) \frac {\partial \sigma_ {p}}{\partial w _ {k}}}{\sigma_ {p} ^ {2}} - \lambda
$$

# Deriving Tangency Portfolio

As the risk-free rate $R_{f}$ is a constant

$$
\frac {\partial (\mu_ {p} - R _ {f})}{\partial w _ {k}} = \frac {\partial \sum_ {i = 1} ^ {N} w _ {i} \mu_ {i}}{\partial w _ {k}} = \mu_ {k}
$$

• Also,

$$
\sigma_ {p} ^ {2} = \sum_ {i = 1} ^ {N} \sum_ {j = 1} ^ {N} w _ {i} w _ {j} \sigma_ {i j}
$$

$$
2 \sigma_ {p} \frac {\partial \sigma_ {p}}{\partial w _ {k}} = 2 \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k}
$$

$$
\frac {\partial \sigma_ {p}}{\partial w _ {k}} = \frac {1}{\sigma_ {p}} \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k}
$$

# Deriving Tangency Portfolio

- Then $\frac{\partial L}{\partial w_k} = \frac{\sigma_p \frac{\partial (\mu_p - R_f)}{\partial w_k} - (\mu_p - R_f) \frac{\partial \sigma_p}{\partial w_k}}{\sigma_p^2} - \lambda = \frac{\sigma_p \mu_k - (\mu_p - R_f) \frac{1}{\sigma_p} \sum_{i=1}^{N} w_i \sigma_{ik}}{\sigma_p^2} - \lambda$   
- Setting $\frac{\partial L}{\partial w_k} = 0$ , we get $\sigma_p \mu_k - (\mu_p - R_f) \frac{1}{\sigma_p} \sum_{i=1}^{N} w_i \sigma_{ik} = \lambda \sigma_p^2$ .... (*).   
From this line, multiply $w_{k}$ to both side and sum $k$ from 1 to $N$

$$
\begin{array}{l} \sigma_ {p} \sum_ {k = 1} ^ {N} w _ {k} \mu_ {k} - (\mu_ {p} - R _ {f}) \frac {1}{\sigma_ {p}} \sum_ {i = 1} ^ {N} \sum_ {k = 1} ^ {N} w _ {i} w _ {k} \sigma_ {i k} = \lambda \sigma_ {p} ^ {2} \sum_ {k = 1} ^ {N} w _ {k} \\ \sigma_ {p} \mu_ {p} - (\mu_ {p} - R _ {f}) \sigma_ {p} = \lambda \sigma_ {p} ^ {2} \\ \lambda = \frac {R _ {f}}{\sigma_ {p}} \\ \end{array}
$$

# Deriving Tangency Portfolio

- Put it in equation $(^{*})$

$$
\sigma_ {p} \mu_ {k} - (\mu_ {p} - R _ {f}) \frac {1}{\sigma_ {p}} \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k} = R _ {f} \sigma_ {p}
$$

$$
\mu_ {k} - R _ {f} = \frac {\mu_ {p} - R _ {f}}{\sigma_ {p} ^ {2}} \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k} \qquad \mathrm {f o r k = 1 , . . . , N}
$$

Denote $z_{i} = \frac{\mu_{p} - R_{f}}{\sigma_{p}^{2}} w_{i}$ , so it becomes $\mu_{k} - R_{f} = \sum_{i=1}^{N} z_{i} \sigma_{ik}$ for $k = 1, \ldots, N$

$$
\left( \begin{array}{c} \mu_ {1} - R _ {f} \\ \vdots \\ \mu_ {N} - R _ {f} \end{array} \right) = \left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) \left( \begin{array}{c} z _ {1} \\ \vdots \\ z _ {N} \end{array} \right)
$$

$$
\underline {{z}} = \Sigma^ {- 1} \left(\underline {{\mu}} - R _ {f} \underline {{I}}\right)
$$

# Deriving Tangency Portfolio

- The equation simply means that the weights are proportional to

$$
\underline {{w}} \propto \Sigma^ {- 1} \left(\underline {{\mu}} - R _ {f} \underline {{I}}\right)
$$

- With the condition $\sum_{i=1}^{N} w_{i} = 1$

$$
\underline {{\boldsymbol {w}}} = \frac {\boldsymbol {\Sigma} ^ {- 1} \left(\underline {{\boldsymbol {\mu}}} - R _ {f} \underline {{\boldsymbol {I}}}\right)}{\underline {{\boldsymbol {I}}} ^ {T} \boldsymbol {\Sigma} ^ {- 1} \left(\underline {{\boldsymbol {\mu}}} - R _ {f} \underline {{\boldsymbol {I}}}\right)}
$$

# Python Example

1. Download stock data for AAPL, MSFT, TSLA, GOOG from 2018-2020   
2. Calculate the minimum variance portfolio   
3. Suppose risk-free rate is $5\%$ , calculate the tangency portfolio

- PyPortfolioOpt (https://pypportfolioopt.readthedocs.io/en/latest/) can be used to solve various portfolio optimization problems

![](images/4fa30c0489a1842800aff364053417f3282e1682f39c770a3ac74c0675e6416a.jpg)

# Python Example

```txt
Minimum Variance Portfolio Weights:  
OrderedDict(['AAPL', 0.18907], ('GOOG', 0.55898), ('MSFT', 0.25195), ('TSLA', 0.0)])  
Tangency Portfolio Weights:  
OrderedDict(['AAPL', 0.45481], ('GOOG', 0.0), ('MSFT', 0.0), ('TSLA', 0.54519])  
Expected annual return: 28.9%  
Annual volatility: 29.5%  
Sharpe Ratio: 0.91  
Expected annual return: 88.2%  
Annual volatility: 46.1%  
Sharpe Ratio: 1.80 
```

```python
import yfinance as yf  
import pandas as pd  
from pypfopt import EfficientFrontier, risk_models, expectedreturns 
```

```txt
Step 1: Download stock data  
tickers = ['AAPL', 'MSFT', 'TSLA', 'GOOG']  
start_date = '2018-01-01'  
end_date = '2020-12-31' 
```

```python
data = yf.download(tickers, start=start_date, end=end_date)[['Close'] # Step 2: Calculate expected returns and covariance matrix mu = expectedreturns.mean_historical_return(data) S = risk_models.sample_cov(data) 
```

```python
Minimum Variance Portfolio ef_mv = EfficientFrontier(mu, S)  
weights_mv = ef_mv.min_volatility()  
cleanedweights_mv = ef_mv.cleanweights()  
print("Minimum Variance Portfolio Weights:")  
print(cleannedweights_mv) 
```

```txt
Step 3: Calculate Tangency Portfolio risk free rate = 0.05 # 5% 
```

```python
ef_tangency = EfficientFrontier(mu, S, weight_bounds=(0, 1))  
weights_tangency = ef_tangency.max_sharpe(risk_free_rate=risk_free_rate)  
cleanedweights_tangency = ef_tangency.cleanweights()  
print("\\nTangency Portfolio Weights:")  
print(cleannedweights_tangency) 
```

```python
Display portfolio performance  
performance_mv = ef_mvportfolio_performance(verbatim=True)  
performance_tangency = ef_tangencyportfolio_performance(verbatim=True) 
```

# ALGOGENE Example

![](images/87c2df81f8c2a18cdbc7c0be67c47301b0d2b803125a5787ff8ece5c1f4804b8.jpg)

# Capital Asset Pricing Model (CAPM)

# Background

- We learnt from previous section how to construct an efficient frontier and determine the tangency portfolio   
- When the number of assets increase, the number of parameters will increase quadratically $O(N^{2})$ . It leads to difficulty to compute for a large portfolio.   
- So a simplified financial model is preferred

# Market Portfolio

- Market portfolio (M) is a portfolio consisting of ALL risky assets in the economy.

- Note that ALL risky assets are not limited to stocks. The market portfolio include stocks, bonds, real estates, human capital etc. It includes both tradable and non-tradable assets.

Each asset is held in a proportion according to its market value:

$$
w _ {i} ^ {*} = \frac {\text {M a r k e t V a l u e o f A s s e t i}}{\text {M a r k e t V a l u e o f A l l A s s e t s}} = \frac {P _ {i} S _ {i}}{\sum_ {j = 1} ^ {N} P _ {j} S _ {j}}
$$

where

- $N =$ number of risky assets in the economy   
- $P_{i} =$ market price of asset i per unit of shares   
- $S_{i} =$ number of outstanding shares of asset i

# Market Portfolio

# A Major Conclusion under Market Equilibrium

- Recall that ALL (risk-averse) investors will hold portfolios formed by the risk-free asset and the tangency portfolio (T).   
- Under market equilibrium, the tangency portfolio (T) [Demand for risky assets] MUST be exactly the same as the market portfolio (M) [Supply for risky assets] in the economy.

# CAPM Assumptions

1. Assets are all tradable and are all infinitely divisible.   
2. All investors are risk-averse and select their portfolios in a single-period time horizon based on the mean-variance criterion (Markowitz assumptions).   
3. All investors have the same belief in expected returns, variances and covariances of all assets (homogeneous expectations).   
4. All investors can do unlimited short selling of risky assets, and borrow or lend money at the same risk-free interest rate.   
5. No taxes, no transaction costs, information is costless and available to all investors.

- Assumptions 2,3,4 imply that all investors obtain the same efficient frontier   
- Using one-fund theorem, every investor hold a portfolio consisting of the tangency portfolio (T) and risk-free asset, with different portfolio weightings

# Capital Market Line (CML)

- For any efficient portfolio on CML,

$$
E \big (R _ {p} \big) = R _ {f} + \frac {E (R _ {M}) - R _ {f}}{\sigma_ {M}} \sigma_ {p} \qquad \mathrm {o r} \qquad \mu_ {p} = R _ {f} + \frac {\mu_ {M} - R _ {f}}{\sigma_ {M}} \sigma_ {p}
$$

![](images/36f710232cdf2a68cbe7ddd9b2bf3e67f3c951149825f8fb6f17ad009249ea38.jpg)

Note: The dashed line represents the efficient frontier of portfolios which consist of all risky assets.

# Deriving CAPM (Sharpe's Approach)

- Consider a portfolio $i$ and market portfolio $M$   
Note that for every portfolio $i$ , it cannot cross the capital market line   
- For a portfolio between $i$ and $M$ , we have

$$
\begin{array}{l} \cdot \mu_ {p} = w \mu_ {i} + (1 - w) \mu_ {M} \\ \cdot \sigma_ {p} = [ w ^ {2} \sigma_ {i} ^ {2} + (1 - w) ^ {2} \sigma_ {M} ^ {2} + 2 w (1 - w) \sigma_ {i j} ] ^ {1 / 2} \\ \end{array}
$$

• Then,

$$
\begin{array}{l} \frac {\partial \mu_ {p}}{\partial w} = \mu_ {i} - \mu_ {M} \\ \frac {\partial \sigma_ {p}}{\partial w} = \frac {w \sigma_ {i} ^ {2} + (1 - 2 w) \sigma_ {i M} - (1 - w) \sigma_ {M} ^ {2}}{\sigma_ {p}} \\ \end{array}
$$

- Consider the slope at M (i.e. $w = 0$ ) $\frac{\partial \sigma_{p}}{\partial w} = \frac{\sigma_{iM} - \sigma_{M}^{2}}{\sigma_{M}}$

![](images/7ee59a3c87c147b5e45d500032ebee0b8fea9006fb3981b35df562be72c05563.jpg)

# Deriving CAPM (Sharpe's Approach)

- Note that the slope of CML at portfolio M is $\frac{\mu_{M} - R_{f}}{\sigma_{M}}$   
So we have

$$
\frac {\mu_ {M} - R _ {f}}{\sigma_ {M}} = \frac {(\mu_ {i} - \mu_ {M}) \sigma_ {M}}{\sigma_ {i M} - \sigma_ {M} ^ {2}}
$$

$$
\frac {\partial \mu_ {p}}{\partial w} / \frac {\partial w}{\partial \sigma_ {p}} \mathrm {w h e n} w = 0
$$

$$
\mu_ {i} = R _ {f} + \frac {\sigma_ {i M}}{\sigma_ {M} ^ {2}} (\mu_ {M} - R _ {f})
$$

• Define $\beta_{i} = \frac{\sigma_{i M}}{\sigma_{M}^{2}}$

CAPM model: $\mu_{i} = R_{f} + \beta_{i}\left(\mu_{M} - R_{f}\right)$

# Security Market Line (SML)

- SML is graphical representation of the CAPM   
- For any asset $i$

Expected market risk premium

$$
E (R _ {i}) = R _ {f} + \underbrace {\beta_ {i} \left(E (R _ {M}) - R _ {f}\right)} _ {\mathrm {r e s p e c i e d}} \qquad \mathrm {o r} \qquad \mu_ {i} = R _ {f} + \beta_ {i} \left(\mu_ {M} - R _ {f}\right)
$$

Expected risk premium

Properties of SML

$\beta_{M} = 1$   
$\beta_{R_f} = 0$   
- Beta of a portfolio $\beta_{p} = \sum_{i=1}^{N} w_{i} \beta_{i}$   
- Every assets lie on SML   
- For 2 assets A and B, $\frac{E(R_A) - R_f}{\beta_A} = \frac{E(R_B) - R_f}{\beta_B} = \frac{E(R_M) - R_f}{1}$

![](images/96d5ce21d4e6290212b334ce5e32404d24de56dfc38c8f5b7db3dbcd88b29b71.jpg)

# CAPM

- Consider a linear regression model

$$
(R _ {i} - R _ {f}) _ {t} = \alpha_ {i} + \beta_ {i} (R _ {M} - R _ {f}) _ {t} + e _ {i, t} \quad \mathrm {t = 1 , 2 , \ldots , T}
$$

We have

$\widehat{\beta}_{i} = \frac{\sum_{t=1}^{T}\left(R_{M,t}-\mu_{M}\right)\left(R_{i,t}-\mu_{i}\right)}{\sum_{t=1}^{T}\left(R_{M,t}-\mu_{M}\right)^{2}} = \frac{\operatorname{Cov}\left(R_{M}, R_{i}\right)}{\operatorname{Var}\left(R_{M}\right)} = \frac{\sigma_{i M}}{\sigma_{M}^{2}}$   
$R^{2} = \frac{\beta_{i}\sigma_{M}^{2}}{\sigma_{i}^{2}} = (\rho_{iM})^{2}$   
- $\sigma_{i}^{2} = \beta_{i}^{2}\sigma_{M}^{2} + \tau_{i}^{2}$

Systematic risk that is non-diversifiable

Non-systematic risk that is diversifiable

![](images/0c828c130c924fb556c21e26da651cafec6fb0ad423e5959182a87947edd5434.jpg)

# Non-systematic risk

- Consider an equally weighted portfolio

$$
R _ {p} = \sum_ {i = 1} ^ {N} \frac {R _ {i}}{N} = \frac {1}{N} \sum_ {i = 1} ^ {N} \left[ R _ {f} + \beta_ {i} (R _ {M} - R _ {f}) + e _ {i} \right] = R _ {f} + \beta_ {p} (R _ {M} - R _ {f}) + \frac {1}{N} \sum_ {i = 1} ^ {N} e _ {i}
$$

- Then,

$$
\sigma_ {p} ^ {2} = \beta_ {p} ^ {2} \sigma_ {M} ^ {2} + \frac {1}{N ^ {2}} \sum_ {i = 1} ^ {N} \tau_ {i} ^ {2}
$$

Tends to 0 if N is large

# The Beta Coefficient

- The beta coefficient is a key parameter in the capital asset pricing model (CAPM).   
- Beta (also referred to as financial elasticity or correlated relative volatility) can be regarded as a measure of the sensitivity of the asset's returns to market returns.   
- For a well diversified portfolio,

- If $\beta_{i} > 1$ , the portfolio is more aggressive than the market portfolio.   
- If $\beta_{i} = 1$ , the portfolio is as aggressive than the market portfolio   
- If $\beta_{i} < 1$ , the portfolio is less aggressive than the market portfolio

# Performance Measures

# Which investment do you prefer?

![](images/1f9fcc4c26c964729b6b81a53d4324c0a945d28090b5f926e2a714fd4c807a6c.jpg)  
Two Equal Return Investments: Which is Better?

# Reasons for Performance Evaluation

- Fund managers want to know   
- how good their portfolio performs   
- the ability to diversify a portfolio effectively   
- Investors want to   
- compare different investment products in the market   
- whether their investment advisors/ fund managers have “value added”

# Performance Measures

1. Sharpe Ratio   
2. Sortino Ratio   
3. Treynor Ratio   
4. Jensen's Alpha   
5. Information Ratio   
6. Maximum Drawdown   
7. Calmar Ratio

# Sharpe Ratio

- A risk-adjusted measure introduced by Sharpe in 1966   
- Risk is measured by standard deviations of portfolio returns   
- Sharpe ratio (SR) is the reward-to-variability ratio for a portfolio.

$$
S R _ {p} = \frac {E (R _ {p}) - R _ {f}}{\sigma_ {p}}
$$

where $E\big(R_p\big)$ and $\sigma_{p}$ are the annualized expected return and standard derivation respectively

• Aim:

- Compare $S R_{p}$ with the slope of the ex-post CML in which

- If $S{R}_{p} > S{R}_{M}$ ,our portfolio beat the market.   
- If $S{R}_{p} < S{R}_{M}$ ,our portfolio lost to the market.

We may also compare $S R_{p}$ with the slope of the peers.

# Sharpe Ratio

- An investment is considered to be sensible if Sharpe ratio is at least 1. i.e.

- for 1 unit of risk taken, you can get at least 1 unit of returns

In the asset management industry,   
It is very common to see funds having Sharpe ratio between 0.5 to 1.5   
- A fund manager that achieve Sharpe ratio between 1.5 to 2.5 is considered to be good (around top $10\%$ in the market)   
- A Sharpe ratio above 2.5 is exceptionally good investment (around the top $1 \%$

# Sharpe Ratio

- We can interpret Sharpe ratio in another way   
- Suppose risk-free rate $R_{f}$ is zero and return is normally distributed   
- Let's say we want Sharpe ratio to be greater than $z$

$$
\frac {E (R _ {p}) - R _ {f}}{\sigma_ {p}} > z \quad \Rightarrow \quad \mu_ {p} - z \sigma_ {p} > 0
$$

- For $z = 1$ , we have $0.5 + 0.6827 / 2 = 84.14\%$ probability for having positive return in a 1-year horizon.   
- For $z = 2$ , we have $0.5 + 0.9545 / 2 = 97.73\%$ probability for having positive return in a 1-year horizon.   
- For $z = 3$ , we have $0.5 + 0.9973 / 2 = 99.87\%$ probability for having positive return in a 1-year horizon.

![](images/3570ee38fdd094cba17604ae1bd71380aa04d362cfbcc7157005d77a2d4eb0ad.jpg)

# Sharpe Ratio - example

- Suppose risk-free rate is $1\%$ . You are given the following monthly returns of a fund. Calculate the Sharpe ratio.

<table><tr><td>Month</td><td>Jan-24</td><td>Feb-24</td><td>Mar-24</td><td>Apr-24</td><td>May-24</td><td>Jun-24</td><td>Jul-24</td><td>Aug-24</td><td>Sep-24</td><td>Oct-24</td><td>Nov-24</td><td>Dec-24</td></tr><tr><td>Return</td><td>1.20%</td><td>2.30%</td><td>-1.50%</td><td>0.80%</td><td>1.70%</td><td>-2.10%</td><td>3.20%</td><td>0.40%</td><td>2.10%</td><td>-0.90%</td><td>1.10%</td><td>0.30%</td></tr></table>

- Answer:

- Average monthly return $= \frac{\left( {{1.2}\%  + {2.3}\%  + \cdots  + {0.3}\% }\right) /{12}} = {0.7167}\%$   
Monthly std dev = $\sqrt{\frac{(1.2\%-0.7167\%)^2+(2.3\%-0.7167\%)^2+\cdots+(0.3\%-0.7167\%)^2}{12-1}}$ = 1.59%   
- Annualized return $= {12}^{ * }{0.7167}\%  = {8.6}\%$   
Annual std dev = 1.59%*√12 = 5.4948%   
- Sharp ratio $= \frac{8.6\% - 1\%}{5.4948\%} = 1.383$

# Sharpe Ratio - example

# Given the following info:

average annual risk-free return was $5 \%$   
average annual return on the S&P 500 was $24.1\%$   
average annual return on Fund A was $20.5\%$   
average annual return on Fund B was $11.8\%$   
- betas of Fund A and Fund B are 1.0 and 0.6, respectively   
standard deviation of S&P 500 was $14\%$   
standard deviation of Fund A was $15\%$   
standard deviation of Fund B was $10\%$

# We evaluate the performance of the two funds

• $S R_{F u n d A} = (20.5 - 5) / 15 = 1.003$   
• $S R_{F u n d B} = (11.8 - 5) / 15 = 0.68$   
• $S R_{S \& P 500} = (24.1 - 5) / 14 = 1.346$

none of the funds can beat the market

# Sharpe Ratio - example

- When the dotted lines of portfolios (Fund A and Fund B) are flatter than the solid line (the ex-post CML for S&P 500), the performance of both portfolios is worse than the market portfolio because the CML represents the efficient allocation of capital between market portfolio and risk-free asset.

![](images/ace886ecdeb3177799c8d81c0eb6277f08aa84da7ac267fb208cc7519357c1be.jpg)

# Sharpe Ratio

- Advantages

- Easy to interpret and calculate   
Useful for comparing different portfolios

- Limitations

- Sensitive to the choice of the risk-free rate and sampling period

# Sortino Ratio

- Sortino Ratio is similar to Sharpe Ratio but focuses on downside risk $\sigma_{d}$

$$
\begin{array}{l} {S o r t i n o R a t i o = \frac {E (R _ {p}) - R _ {f}}{\sigma_ {d}}} \\ {\mathrm {w h e r e}} \end{array} \left\{ \begin{array}{l} {\mu_ {d} = \frac {1}{n} \sum_ {i = 1} ^ {n} \min (0, r _ {i})} \\ {\sigma_ {d} = \sqrt {\frac {1}{n - 1} \sum_ {i = 1} ^ {n} [ \min (0 , r _ {i}) - \mu_ {d} ] ^ {2}}} \end{array} \right.
$$

- Aim: choose the portfolio with the highest Sortino Ratio

# Sortino Ratio - example

- Assume risk free rate $= 0\%$   
Given a return series $= \{1\%, 1.2\%, -0.5\%, -1.6\%, 2.1\% \}$   
- Downside return series = $\{0\%, 0\%, -0.5\%, -1.6\%, 0\%\}$

We have

$$
\begin{array}{l} \cdot E \left(R _ {p}\right) = \frac {1 \% + 1.2 \% - 0.5 \% - 1.6 \% + 2.1 \%}{5} = 0.0044 \\ \cdot \mu_ {d} = \frac {0 + 0 - 0 . 5 \% - 1 . 6 \% + 0}{5} = - 0. 0 0 4 2 \\ \cdot \sigma_ {d} = \sqrt {\frac {(0 + 0 . 0 0 4 2) ^ {2} + (0 + 0 . 0 0 4 2) ^ {2} + (- 0 . 5 \% + 0 . 0 0 4 2) ^ {2} + (- 1 . 6 \% + 0 . 0 0 4 2) ^ {2} + (0 + 0 . 0 0 4 2) ^ {2}}{4}} = 0. 0 0 6 9 4 2 6 2 \\ \cdot \text {S o r t i n o R a t i o} = \frac {0 . 0 0 4 4 - 0}{0 . 0 0 6 9 4 2 6 2} = 0. 6 3 3 8 \\ \end{array}
$$

# Sortino Ratio

Pros:   
Take into account of downside risk   
- Cons:   
- Also sensitive to the choice of risk-free rate and sampling period

# Treynor Ratio (1965)

- Similar to Sharpe Ratio but risk is measured by portfolio beta

$$
T r e y n o r R a t i o = \frac {E (R _ {p}) - R _ {f}}{\beta_ {p}}
$$

• Aim:

- Choose the portfolio with the highest Treynor Ratio   
- Compare $T R_{p}$ with the slope of the ex-post SML

If $TP_{p} > TP_{M}$ , our portfolio beat the market.   
If $TP_{p} < TP_{M}$ , our portfolio lost to the market.

We may also compare $T P_{p}$ with the slope of the peers.

# Treynor Ratio - example

# Given the following info:

average annual risk-free return was $5 \%$   
average annual return on the S&P 500 was $24.1\%$   
average annual return on Fund A was $20.5\%$   
average annual return on Fund B was $11.8\%$   
- betas of Fund A and Fund B are 1.0 and 0.6, respectively   
standard deviation of S&P 500 was $14\%$   
standard deviation of Fund A was $15\%$   
standard deviation of Fund B was $10\%$

# We evaluate the performance of the two funds

• $T R_{F u n d A} = (20.5\% - 5\%) / 1 = 0.155$   
• $T R_{F u n d B} = (11.8\% - 5\%) / 0.6 = 0.113$   
- $TR_{S\& P500} = (24.1\% - 5\%) / 1 = 0.191$

none of the funds can beat the market

# Treynor Ratio - example

- When the dotted lines of portfolios (Fund A and Fund B) are flatter than the solid line (the ex-post SML for S&P 500), the performance of both portfolios is worse than the market index.

![](images/477064c4920e9ac7d1cd5c6ef41d1e039494b3e1d4b3e5a160d919d06b75c0fb.jpg)

# Jensen's Alpha (1968)

- Jensen's Alpha (from the SML)

$$
\alpha_ {p} = E \big (R _ {p} \big) - \big \{R _ {f} + \beta_ {p} \big [ E (R _ {M}) - R _ {f} \big ] \big \}
$$

Expected return from CAPM

- $\alpha_{p}$ can be estimated from

$$
(R _ {p} - R _ {f}) _ {t} = \alpha_ {p} + \beta_ {p} (E (R _ {M}) - R _ {f}) _ {t} + e _ {p, t}
$$

We can test for $H_{0}$ : $\alpha_{0} = 0$   
If $\alpha_0 > 0$ , our portfolio beat the market   
If $\alpha_0 < 0$ , our portfolio beat the market

# Jensen's Alpha - example

# Given the following info:

average annual risk-free return was $5 \%$   
average annual return on the S&P 500 was $24.1\%$   
average annual return on Fund A was $20.5\%$   
average annual return on Fund B was $11.8\%$   
- betas of Fund A and Fund B are 1.0 and 0.6, respectively   
standard deviation of S&P 500 was $14\%$   
standard deviation of Fund A was $15\%$   
standard deviation of Fund B was $10\%$

# We evaluate the performance of the two funds

- $\alpha_{Fund A} = 20.5\% - [5\% + 1(24.1 - 5)] = -3.6\%$ .   
- $\alpha_{\text {Fund } B} = 11.8\% - [5\% + 0.6(24.1 - 5)] = -4.66\%$

none of the funds can beat the market

# Jensen's Alpha - example

- When the points of portfolios (Fund A and Fund B) are below the ex-post SML for S&P 500, the performance of both portfolios is worse than the market index.

![](images/25965b232fa8acc0ce35488e5c100a4c32db7a70938c1862c3ed9ef8626bc15d.jpg)

# Jensen's Alpha

- Note that Treynor Ratio and Jensen's Alpha can be illustrated in return-beta plane.

![](images/b33111658187dd671dc9dee73b724827f8af2938517d7723aea90d90f0d661d6.jpg)

# Comparing the Measures

• $T R_{p}$ and $\alpha_{p}$ usually give the same sign and ranking.   
- It is possible that $S{R}_{p}$ indicate our portfolio lost to the market while $T{R}_{p}$ and ${\alpha }_{p}$ indicate that our portfolio beat the market because the presence of non-systematic risk in our portfolio.   
- $SR_{p}$ can rank a set of portfolios differently than $TR_{p}$ and $\alpha_{p}$ since they use different risk measures.   
- All the measures may not be meaningful if $E(R_{M}) < R_{f}$ since both the SML and CML will be downward sloping.   
- All the measures have been criticized for the subjectivity in the choice of the market index and the risk-free rate.

# Information Ratio

- Other than comparing with the risk-free rate $R_{f}$ , we can also choose a benchmark rate of return, $R_{b}$ , for comparison.   
- Define the tracking error

$$
T E = \sigma (R _ {p} - R _ {b})
$$

- Then the information ratio (or appraisal ratio) is

$$
I R = \frac {E (R _ {p} - R _ {b})}{\sigma (R _ {p} - R _ {b})}
$$

- Aim: choose the portfolio with the largest Information Ratio

# Information Ratio

- Suppose the benchmark is the portfolio on SML with the same beta and its expected return is

$$
E (R _ {b}) = R _ {f} + \beta_ {p} [ E (R _ {p}) - R _ {f} ]
$$

Using $R_{p} - R_{f} = \alpha_{p} + \beta_{p}\big[R_{M} - R_{f}\big] + e_{p}$ with $\operatorname {E}(e_p) = 0$ and $\operatorname {Var}\bigl (e_p\bigr) = \tau_p^2$   
Therefore,

$$
\left\{\begin{array}{r l r}E (R _ {p} - R _ {b}) = \alpha_ {p}&&\rightarrow I R = \frac {\alpha_ {p}}{\tau_ {p}}\\\sigma (R _ {p} - R _ {b}) = \tau_ {p}&&\end{array}\right.
$$

# Information Ratio

- Using information ratio, evaluate the performance of the two funds:   
- Recall that

$$
\alpha_ {F u n d A} = 20.5 \% - [ 5 \% + 1 (24.1 - 5) ] = -3.6 \%
$$

$$
\alpha_ {F u n d B} = 11.8 \% - [ 5 \% + 0.6 (24.1 - 5) ] = -4.66 \%
$$

• Also, $\sigma_{p}^{2} = \beta_{p}^{2} \sigma_{M}^{2} + \tau_{p}^{2}$ . So we have

$$
\tau_ {F u n d A} = \sqrt {1 5 ^ {2} - 1 ^ {2} 1 4 ^ {2}} = 5. 3 8 5 2
$$

$$
\tau_ {F u n d B} = \sqrt {10 ^ {2} - 0.6 ^ {2} 14 ^ {2}} = 5.4259 \%
$$

Therefore,

$$
I R _ {F u n d A} = \frac {- 3 . 6}{5 . 3 8 5 2} = - 0. 6 6 8 5
$$

$$
I R _ {F u n d B} = \frac {- 4 . 6 6}{5 . 4 2 5 9} = - 0. 8 5 8 8
$$

$\rightarrow$ Fund A is better than Fund B, but both perform worse than the benchmark

# Information Ratio

# Remarks:

- If $R_{p} - R_{f} = \alpha_{p} + \beta_{p}\left[R_{M} - R_{f}\right] + e_{p}$ is fitted using monthly data, we obtain monthly $IR =$

- Annualized $IR = \frac{12\alpha_p}{\sqrt{12}\tau_p} = \sqrt{12} IR$

- Grinold and Kahn (2000, Active Portfolio Management, McGrawHill, 2nd Ed) argued that reasonable IR should range from 0.50 to 1.00.

Good: IR > 0.5   
- Exceptional: IR > 1.0

# Maximum Drawdown

- Maximum Drawdown (MDD), expressed as a percentage, measures the largest peak-to-trough decline in the value of an investment portfolio   
- Indicates potential risk and the worst-case scenario for an investor.   
- Useful for assessing the volatility and risk of an investment strategy.   
- Formula:

$$
M D D = \frac {P _ {p e a k} - P _ {t r o u g h}}{P _ {p e a k}}
$$

where $P_{peak} = \text{Maximum value (peak) of the portfolio}$ $P_{trough} = \text{Minimum value (trough) following the peak}$

# Maximum Drawdown

![](images/f6f6a101506ed9c7f2c0abccf145a0dab8d89c64546732d24bd6b9080ff7d8d5.jpg)  
Time

# Maximum Drawdown - example

- Suppose the daily portfolio value is [100, 120, 90, 110, 80, 130, 70, 150]

import numpy as np   
def calculate_mdd(data): portfolio_values $\equiv$ np.array(data) #Calculate the running maximum running_max $=$ npmaximumaccumulate.portfolio_values) #Calculate drawdowns drawdowns $=$ (running_max - portfolio_values)/running_max #Maximum Drawdown max_drawdown $=$ np.max drawnowns return max_drawdown   
mdd $=$ calculate_mdd([100, 120, 90, 110, 80, 130, 70, 150])   
print(f"Maximum Drawdown: {mdd:.}")

Maximum Drawdown: $46.15\%$

# Calmar Ratio

- The Calmar Ratio measures the performance of an investment relative to its maximum drawdown.   
- Formula:

$$
C a l m a r R a t i o = \frac {A n n u a l i z e d R e t u r n}{\left| M a x . D r a w d o w n \right|}
$$

Pros: risk adjusted metric that focus on downside risk   
- Cons: ignore the frequency of drawdown or volatility within the period

# Calmar Ratio - Example

• Given a portfolio's monthly value: [100, 120, 90, 110, 80, 130, 70, 150], what is the Calmar Ratio?   
- Answer:

- Max. Drawdown = 46.15%   
- Annualized Return = (20% - 25% + 22.22% +...+ 114.29%) / 7*12 = 206.71%   
- Thus, Calmar Ratio = 206.71%/ 46.15% = 4.48

<table><tr><td>Month</td><td>Portfolio Value</td><td>Monthly Return</td><td>Drawdown</td></tr><tr><td>1</td><td>100</td><td></td><td>0.00%</td></tr><tr><td>2</td><td>120</td><td>20.00%</td><td>0.00%</td></tr><tr><td>3</td><td>90</td><td>-25.00%</td><td>25.00%</td></tr><tr><td>4</td><td>110</td><td>22.22%</td><td>8.33%</td></tr><tr><td>5</td><td>80</td><td>-27.27%</td><td>33.33%</td></tr><tr><td>6</td><td>130</td><td>62.50%</td><td>0.00%</td></tr><tr><td>7</td><td>70</td><td>-46.15%</td><td>46.15%</td></tr><tr><td>8</td><td>150</td><td>114.29%</td><td>0.00%</td></tr></table>

# Choosing an Appropriate Measure

- The criterion to choose an appropriate performance measure depends on the investment assumptions.

- If the portfolio represents our entire risky investment fund, use Sharpe, Sortino or Calmar Ratio.   
- For an actively managed portfolio with benchmark comparison, use Information ratio   
- If the portfolio is one of many portfolios combined into a large investment fund, use the Jensen's alpha or the Treynor ratio but the Treynor ratio is more complete because it adjusts for risk.

# Inclusion of a new asset

- If we are holding a portfolio $P$ , we will add the new asset $Q$ to our portfolio if

$$
S R _ {Q} > S R _ {P} C o r r (R _ {Q}, R _ {P})
$$

![](images/9fdfd563f76da05f371b4c658cbb8cee95c7d65065ad3bb7fa7bbe262aea1a51.jpg)

# Stop Loss & Risk Limit Control

# Stop Loss

- Minimizes losses on losing trades.   
• Removes emotional decision-making during losses.   
- Types of Stop Loss

1. Fixed Stop Loss   
2. Trailing Stop Loss   
3. Dollar Risk Approach   
4. Portfolio level Stop Loss

Trade level

Already discussed in L6

# Fixed Stop Loss

- Set the stop loss at a specific price level.   
• For example, we buy 1 share of a stock at $100 and set stop loss at$ 90.

- When price drops significantly, our position will be automatically closed at \(90.   
- The maximum loss of this trade is capped at $10.

# Fixed Stop Loss

- Implement on ALGOGENE (https://algogene.com/TechDoc#PlaceOrder)   
- Eg. We set TP and SL at $10\%$ from the current mid price

```python
def on_marketdatafeed(self, md, ab):  
    sl = md.midPrice*0.9  
    tp = md.midPrice*1.1  
    order = AlgoAPIUtil.OrderObject(instrument = md.instrument, openclose = 'open', buysell = 1, #1=buy, -1=sell  
    ordertype = 0,  
    volume = 1.  
    stopLossLevel = sl,  
    takeProfitLevel = tp  
)  
self.evt.sendOrder(order) 
```

# Trailing Stop Loss

- A trailing stop loss is a dynamic order that adjusts as the price of an asset moves in your favor, locking in profits while limiting potential losses.   
- Mechanism: (assume buy order)

- Set at a specific percentage or amount below the market price.   
- As the price increases, the stop loss level moves up, but it does not move down.

- Benefits:

- Protects profits while allowing for potential gains as the price increases.   
- Reduces the need for constant monitoring.

![](images/30fa209ea2170f2dfd3fd113a7a1b9f36a46b48729156b97e212def6ec7b5d43.jpg)

# Trailing Stop Loss

Example:

- Initial Position: Buy 100 shares of XYZ stock at $50.   
- Trailing Stop Loss: Set at $5 below the highest price reached.

- Price Movement:

• If XYZ rises to $55, the trailing stop loss adjusts to $50 (i.e. $55 - $5).   
- If XYZ then rises to $60, the trailing stop loss moves up to $55.   
- If XYZ drops to $55, the order triggers a sell, locking in a $5 profit per share.

- Outcome:

- Final Sell Price: $55 (when trailing stop is triggered).   
Profit: $(\$ 55 - \$ 50) $x 100 shares = \$ 500.

# Trailing Stop Loss

- Implement on ALGOGENE (https://algogene.com/TechDoc#PlaceOrder)   
- Eg. We set trailing stop at $5 \%$ from its highest value

def on_marketdatafeed(self, md, ab):

```python
order = AlgoAPIUtil.OrderObject( instrument = md.instrument, openclose = 'open', buysell = 1, #1=buy, -1=sell ordertype = 0, volume = 1, trailingstop = 0.05  
) self.evt.sendOrder(order) 
```

# Portfolio Stop Loss

- When a portfolio involves multiple positions, instruments, or trading strategies, setting only a trade level stop loss will not be sufficient.   
- A portfolio stop loss acts as a $2^{\text{nd}}$ layer protection for overall capital.

![](images/3b0640af1b7751b9a031578e5f83b6a9ca3cad517a2c4fbdd7013725af93a344.jpg)

# Portfolio Stop Loss

- Implement on ALGOGENE

(https://algogene.com/TechDoc#UpdatePortfolioStop)

- Eg. We set portfolio stop at $10\%$ from its highest value

class AlgoEvent:

def __init__(self):

pass

def start(self, mEvt):

self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)

self.evt.update portfolioslsl $\mathsf{sl} = 0.1$ ,resume_after $= 60^{*}60^{*}24^{*}7$

self.evt.start()

<table><tr><td>&#x27;self.evt.update portfoliosl&#x27; INPUT
PARAMETER</td><td>IS
NECESSARY</td><td>DATA
TYPE</td><td>DESCRIPTIONS</td><td>SAMPLE</td></tr><tr><td>sl</td><td>Yes</td><td>float</td><td>• the stoploss (in percentage) from a portfolio NAV&#x27;s high water mark
• value between 0 to 1. eg. 0.1 refers to cutting all positions when a portfolio NAV drops 10% from its previous highest level.
• set the value to 0 to disable this feature
• once this stoploss condition trigger, the high water level will be reset to the latest NAV</td><td>0.1</td></tr><tr><td>resume_after</td><td>No</td><td>float</td><td>• when the portfolio stoploss event trigger, this parameter refers to the &quot;cooling period&quot; before resuming the algo
• unit in second, and default is 0</td><td>60*60*24</td></tr></table>

# Risk Limit Stoploss

- Risk managers usually apply additional limits for traders

Daily stop loss   
Weekly stop loss   
Monthly stop loss   
• Quarterly stop loss   
- Yearly stop loss   
VaR Limit   
- DV01 Limit   
- Delta Limit   
0

# Risk Limit Stoploss

- Implement on ALGOGENE (https://algogene.com/TechDoc#UpdateRiskLimitStopLoss)   
- Eg. We set $5 \%$ monthly and $10 \%$ yearly stop loss

class AlgoEvent:

def __init__(self):

pass

def start(self, mEvt):

self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)

self.evt.update_risk_limit_sl(sl=0.05, risk_type='m')  
self.evt.update_risk_limit_sl(sl=0.1, risk_type='y')

self.evt.start()

<table><tr><td>&#x27;self.evt.update_risk_limit_sI&#x27; INPUTPARAMETER</td><td>ISNECESSARY</td><td>DATATYPE</td><td>DESCRIPTIONS</td><td>SAMPLE</td></tr><tr><td>sl</td><td>Yes</td><td>float</td><td>• the stoploss (in percentage) from the portfolio NAV at a period start • value between 0 to 1. eg. 0.1 refers to cutting all positions when a portfolio NAV drops 10% from its previous highest level.• set the value to 0 to disable this feature • once this stoploss condition trigger, the high water level will be reset to the latest NAV</td><td>0.1</td></tr><tr><td>risk_type</td><td>Yes</td><td>string</td><td>• set the risk limit type • &#x27;d&#x27; for daily stop loss limit • &#x27;w&#x27; for weekly stop loss limit • &#x27;m&#x27; for monthly stop loss limit • &#x27;q&#x27; for quarterly stop loss limit • &#x27;y&#x27; for yearly stop loss limit</td><td>&#x27;m&#x27;</td></tr></table>

# Key Takeaways

We learnt how to calculate the overall risk and return of an investment portfolio   
- The lower the correlation, the better the diversification effect will be   
- Efficiency Frontier represents the set of optimal and feasible solutions for asset allocation   
We learnt how to derive the Minimum Variance Portfolio, and Tangency portfolio   
- Capital Asset Pricing Model (CAPM) is a simplified model to relate an asset and the market   
- A rational investor should allocate on the Capital Market Line (CML)   
- Security Market Line (SML) is a graphical presentation of CAPM   
- Different performance measures are introduced for investment comparison   
- Introduce different stop loss and risk limit control mechanism