## 表格

````
```
Look! You can see my backticks.
```
````

```ruby
require 'redcarpet'
markdown = Redcarpet.new("Hello World!")
puts markdown.to_html
```

- [x] #739
- [ ] https://github.com/octo-org/octo-repo/issues/740
- [ ] Add delight to the experience when all tasks are complete :tada:

Table of Number Theory Functions
--------------------------------

The following table shows information about a few important functions
in number theory.

| Name                     | Notation       | First few values                      | Multiplicative property   |
| ------------------------ | -------------- | ------------------------------------- | ------------------------- |
| Möbius function          | $ \mu(n) $     | $ 1, -1, -1, 0, -1 $                  | Multiplicative            |
| Euler's totient function | $ \varphi(n) $ | $ 1, 1, 2, 2, 4 $                     | Multiplicative            |
| Mangoldt function        | $ \Lambda(n) $ | $ 0, \log 2, \log 3, \log 2, \log 5 $ | Not multiplicative        |
| Liouville's function     | $ \lambda(n) $ | $ 1, -1, -1, 1, -1 $                  | Completely multiplicative |

## 代码验证。

```swift reds
LogChannel(n"DEBUG", "hello world");
```

```swift
print("Hello, World!") 
```

```rust
fn main() { println!("hello world"); }
```

```lua
print("Hello World")
```

```cpp
#include <iostream>

int main() {
    std::cout << "Hello World!";
    return 0;
}
```

```yaml
some:
  interesting:
    - property
```

```json
{
  "some": { "interesting": ["property"] }
}
```

```xml
<some>
  <interesting>
    <property />
  </interesting>
</some>
```

## 公式验证。

- 扩展欧几里得算法（Python）

$\begin{aligned}
& \texttt{def ex_gcd(a, b):}\\
& \qquad \texttt{if b == 0: }\\
& \qquad \qquad \texttt{return 1, 0, a}\\
& \qquad \texttt{else: }\\
& \qquad \qquad \texttt{x, y, g = ex_gcd(b, a % b)}\\
& \qquad \qquad \texttt{return y, x - (a // b) * y, g}\\
\\
\\
& \texttt{N = 5}\\
& \texttt{for n in range(1, N + 1):}\\
& \qquad \texttt{for m in range(1, N + 1):}\\
& \qquad \qquad \texttt{print(ex_gcd(n, m))}
\end{aligned}$

- 范德蒙德行列式

$$\begin{vmatrix}
1   & 1   & \dots & 1\\
x_1 & x_2 & \dots & x_n\\
\vdots & \vdots & \ddots & \vdots\\
x_1^{n-1} & x_2^{n-2} & \dots & x_n^{n-1}
\end{vmatrix} = \prod_{1\le j\le i\le n} (x_i-x_j)$$

- 碳酸氢钙溶液加热

$$\mathrm{Ca(HCO_3)_2\xrightarrow{\triangle}CaCO_3\downarrow+CO_2\uparrow+H_2O}$$

- 薛定谔方程

$$i\hbar\dfrac{\partial}{\partial t}|\psi (t)\rangle = \hat{H}|\psi(t)\rangle$$

$$i\hbar\dfrac{\partial}{\partial t}\Psi (x,t) = \left[-\dfrac{\hbar^2}{2m}\dfrac{\partial^2}{\partial x^2}+V(x,t)\right]\Psi(x,t)$$


- 麦克斯韦方程组

$$\begin{cases}
\nabla \cdot \mathbf{E} = \dfrac{\rho}{\varepsilon_0} & \text{Gauss's law}\\
\nabla \cdot \mathbf{B}=0 & \text{Gauss's law for magnetism}\\
\nabla \times \mathbf{E} = -\dfrac{\partial\mathbf{B}}{\partial t} & \text{Maxwell–Faraday equation}\\
\nabla \times \mathbf{B} =  \mu_0 \left(\mathbf{J} + \varepsilon_0\dfrac{\partial \mathbf{E}}{\partial t}\right) & \text{Ampère's circuital law}
\end{cases}$$


- 异或运算的真值表

$$\begin{array}{|l|l|l|}
\hline
x & y & S\\
\hdashline
0 & 0 & 0\\
\hline
0 & 1 & 1\\
\hline
1 & 0 & 1\\
\hline
1 & 0 & 0\\
\hline
\end{array}$$

- 狄利克雷函数

$$ \mathbf{1}_{\mathbb{Q}}(x)=
{\begin{cases}
1&x\in \mathbb {Q} \\
0&x\notin \mathbb {Q}
\end{cases}}$$

- 雅可比符号

$$\left(\dfrac{a}{p}\right) =
\begin{cases}
0 & \text{if $a\equiv 0\pmod{p}$}\\
1 & \text{if }a\not\equiv 0\pmod p \wedge \exists  x: x^2\equiv a\pmod p \\
-1 & \text{if }a\not\equiv 0\pmod p \wedge \nexists  x: x^2\equiv a\pmod p
\end{cases}$$

- 斯托克斯公式

$$\oint_{\Gamma}Pdx+Qdy+Rdz=\iint_{\Sigma}\left(\dfrac{\partial R}{\partial y}-\dfrac{\partial Q}{\partial z}\right) dydz+\left(\dfrac{\partial P}{\partial z}-\dfrac{\partial R}{\partial x}\right) dzdx+\left(\dfrac{\partial Q}{\partial x}-\dfrac{\partial P}{\partial y}\right) dxdy$$

- 涛函数与黎曼函数

$$\begin{aligned}
\Gamma(z)=&\int_{0}^{\infty} t^{z-1} \mathrm{e}^{-t}dt &\\
\zeta(s)=&\sum_{n=1}^{\infty} \dfrac{1}{n^s}
\end{aligned}$$

- 德摩根律

$$\begin{align}
& \neg (P\lor Q)\Longleftrightarrow \neg P\land\neg Q \tag{propositional logic}\\
& \neg (P\land Q)\Longleftrightarrow \neg P\lor\neg Q \\
& (A \cup B)^{\complement}=A^{\complement}\cap B^{\complement} \tag{set theory}\\
& (A \cap B)^{\complement}=A^{\complement}\cup B^{\complement} \\
& \overline{A\cup B}=\overline{A}\cap\overline{B} \tag{probability theory}\\
& \overline{A\cap B}=\overline{A}\cup\overline{B}
\end{align}$$

- 贝叶斯公式

$$P(A_i|B)=\dfrac{P(B|A_i)\cdot P(A_i)}{\sum_{j=1}^n P(B|A_j)\cdot P(A_j)}$$

- 麦克劳林级数

$$\begin{align}
&e^x=\sum_{n=0}^{\infty} \dfrac{x^n}{n!}\\
&\sin x = \sum_{n=0}^{\infty} (-1)^n\dfrac{x^{2n+1}}{(2n+1)!}\\
&\cos x = \sum_{n=0}^{\infty} (-1)^n\dfrac{x^{2n}}{(2n)!}\\
&\ln(x+1) = \sum_{n=0}^{\infty} (-1)^n\dfrac{x^{n+1}}{(n+1)!}\\
\end{align}$$

- 自然对数e的定义

$$e=\lim_{x\rightarrow\infty}\left(1+\dfrac{1}{x}\right)^x$$

- 二项式定理

$$(x+y)^n=\sum_{k=0}^n \dbinom{n}{k} x^ky^{n-k}$$