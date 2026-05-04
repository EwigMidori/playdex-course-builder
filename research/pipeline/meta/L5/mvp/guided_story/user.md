请把下面的材料重写成一个 lesson 级 `guided_story` JSON。

目标：
- 面向手机端逐屏点击学习
- 先把 lecture 拆成多个自然 step，再为每个 step 写逐屏内容
- 不要像课件提纲
- 不要像老师讲稿
- 要像一段有推进感的字幕流

目标语言：
zh-CN

course_id：
comp7415a

course_title：
COMP7415A Algorithmic Trading

chapter_id：
lecture-5

chapter_title：
Statistical Arbitrage and Pairs Trading

lesson_id：
L5

本次 step 的主题边界：
full lecture; split into 4-8 natural learning steps unless the source is very short

输出风格：
- 每个 screen 默认 1 到 2 行
- 一节复杂 lecture 通常生成 4 到 8 个 steps
- 每个 step 通常 6 到 14 个 screens，避免 25+ screens 的巨型 step
- 每个 step 只覆盖一个自然学习单元
- 顶层必须优先输出 `{ "lesson_id": ..., "mode": "guided_story", "steps": [...] }`
- 每个 `steps[]` item 必须包含 `sequence_id`、`concept`、`step`
- `steps[].step` 必须符合 guided_story step schema，并包含自己的 `screens` 与 `term_catalog`
- 每行尽量短
- 每 3 到 6 个 screen 插入一次小练习或认知拐点
- 新概念首次出现必须加粗
- 新术语必须使用 `<term id="...">主显示文本</term>` 内嵌标记
- 第二语言术语、英文原词、别名不要直接混写在正文里，放进 `term_catalog.aliases`
- 如果有关键公式，先讲直觉，再给公式
- 如果有关键图，才使用 `media`
- 正文不要写标题
- 正文和练习都不要提“原材料”“材料里”“本节”“这一步”“上一段”等元叙述

额外约束：
- 去掉广告、无关宣传、课程管理信息、教师信息
- 遇到 OCR 噪声时做最小修复
- 不要一次性引入太多新词
- 学生看完这个 step 后，应该能带走一句 punchline
- 输出必须像成品内容，而不是像“基于某份材料整理”的内容
- 填空题答案不要混放多种语言版本
- 不要生成“纯英文术语 + 下一句松散意译”这种不完整教学表达
- 输出时请生成顶层 `term_catalog`
- 如果输出 lesson 级对象，`term_catalog` 放在每个 `steps[].step` 内
- 每个首次出现的术语都要：
  - 在 `lines` 中用 `<term>` 标记
  - 在 `term_catalog` 中登记 `display`、`aliases`、`gloss`

自检清单：
- 学生只看这一份 JSON，是否能自然理解？
- 有没有任何句子泄露“这段话来自源材料”？
- 有没有“根据材料”“原文提到”“这一页”之类的幕后措辞？
- 有没有课程管理信息或备课口吻？
- 术语写法是否统一？
- 填空题答案是否只有必要的、同语言体系内的正确答案？
- 是否所有术语都通过 `<term>` + `term_catalog` 表达，而不是靠自由文本混排？

源材料：

<RELATED_IMPORTANT>
COMP7415A - Mastering the markets: Financial analytics and algorithmic trading
Semester 2, 2024-25
Professor	
Tony Lam
Syllabus	Algorithmic trading is a trending investment approach nowadays that consists of identification of trading opportunities and implementation via computer algorithms. This course will cover emerging trend in the quantitative investment field, and introduce various data analysis techniques and methodologies that are commonly employed in the industry.

The first half of the course focuses on financial data analysis and strategy implementation. The second half of the course discusses practical techniques to manage and deploy algorithmic trading strategies in real financial world.
Introduction by Professor	 
Learning Outcomes	
Course Learning Outcomes	
CLO1. A solid understanding of the nuances of algorithmic trading, design and implement algorithmic trading strategies	
CLO2. The ability to apply quantitative methods to analyze financial data and build financial models	
CLO3. The ability to formulate trading strategies, carry out backtesting, optimization, risk management and interpret investment performance	
CLO4. The ability to apply various investment theories and trading techniques to the real financial market	
CLO5. Familiar with the current trends, and understand the limitations and challenges in the field	
CLO6. Complete a capstone project that includes a full cycle of trading strategy development	
Pre-requisites	To succeed in this course, students are expected to have a foundation and basic knowledge in the following areas:
- Python programming
- Statistics and probability theory
- Mathematics and optimization theory
Compatibility	-
Topics covered	
Course Content	No. of Hours	Course Learning Outcomes
Algorithmic trading basics and financial markets	 	 
Data scraping and database management with Python	 	 
Building backtest framework and rule-based trading strategy	 	 
Statistical time series analysis for market classification	 	 
Statistical arbitrage and pairs trading	 	 
Capital and Risk Management	 	 
Performance measures and portfolio optimization	 	 
Order book and high frequency data modeling	 	 
Market practice in broker selection and account connection	 	 
Machine learning use cases in algorithmic trading	 	 
 
Assessment	
Description	Type	Weighting *	Tentative Assessment Period /
Examination Period ^	Course Learning Outcomes
Written assignment and project	Continuous Assessment	50%	-	 
Written examination covering all the taught contents in the course	Written Examination	50%	-	 
* The weighting of coursework and examination marks is subject to approval
^ The exact examination date uses to be released when all enrolments are confirmed after add/drop period by the Examinations Office.  Students are obliged to follow the examination schedule.  Students should NOT enrol in the course if they are not certain that they will be in Hong Kong during the examination period.  Absent from examination may result in failure in the course. There is no supplementary examination for all MSc curriculums in the Faculty of Engineering.
Course materials	
 

Session dates	
Date	Time	Venue	Remark
Session 1	5 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 2	12 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 3	19 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 4	26 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 5	5 Mar 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 6	19 Mar 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 7	26 Mar 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 8	2 Apr 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 9	9 Apr 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 10	16 Apr 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
LE - Library Extension Building
Add/drop	20 January, 2025 - 12 February, 2025
Maximum class size	146
Back

</RELATED_IMPORTANT>

<PLAIN_TEXT>
         L5: Statistical Arbitrage and 
                   Pairs Trading
                     Course Code: COMP7415
  Lecturer: Tony Lam
    Agenda
    • Statistical arbitrage
      • Pair Trading
      • Co-integration
    • Correlation vs Cointegration
    • Implement a cointegration strategy
    • FX Market Introduction
    • USD-HKD arbitrage
    Introduction to Statistical Arbitrage
    • A trading strategy that seeks to exploit statistical mis-pricings of one or more 
    assets based on historical data.
    • It is a mean reversion strategy that profits from the convergence of asset prices.
    • Commonly involves pairs trading, basket trading, or market-neutral strategies.
    • Due to market-neutral, it reduces exposure to overall market risk of a portfolio.
         Top Hedge Funds
         • Renaissance Technologies: Known for 
           using statistical arbitrage strategies in 
           their Medallion Fund.
         • Two Sigma: Utilizes machine learning 
           and statistical models to implement 
           market-neutral strategies.
   Source: https://www.visualcapitalist.com/growth-of-100-invested-in-jim-simons-medallion-fund/#google_vignette
      Pair Trading
    Introduction to Pair Trading
    • A market-neutral trading strategy that involves 
    trading two correlated assets.
    • Exploits the relative price movements 
    between the two assets.
    • Example: Buying one stock and simultaneously 
    selling another if their price spread deviates 
    from the historical mean.
             Example: Coca-Cola (KO) and PepsiCo (PEP)
           import pandas as pd                                                                 Price          Close      High       Low      Open   Volume
           import yfinance as yf                                                               Ticker            KO        KO        KO        KO       KO
                                                                                               Date
                                                                                               1970-01-02  0.176391  0.176391  0.175051  0.176391  1104000
           # Fetch historical data                                                             1970-01-05  0.173710  0.175855  0.173174  0.175855   556800
           data_KO=yf.download('KO',start='1970-01-01', end='1999-12-31')                      1970-01-06  0.175319  0.175855  0.172906  0.173710   796800
                                                                                               1970-01-07  0.176391  0.176927  0.174247  0.175319   700800
           data_PEP=yf.download('PEP',start='1970-01-01', end='1999-12-31')                    1970-01-08  0.176659  0.177999  0.175319  0.176391  1065600
                                                                                               Price          Close      High       Low      Open  Volume
           # Display the first few rows of the dataset                                         Ticker           PEP       PEP       PEP       PEP     PEP
           print(data_KO.head())                                                               Date
           print(data_PEP.head())                                                              1972-06-01  0.381617  0.381617  0.377205  0.377205  318600
                                                                                               1972-06-02  0.380514  0.385478  0.380514  0.381617  140400
                                                                                               1972-06-05  0.376090  0.382174  0.368900  0.381621  469800
           # Preprocess the data                                                               1972-06-06  0.372771  0.372771  0.371665  0.372771  140400
           data_KO['Date'] = data_KO.index                                                     1972-06-07  0.370559  0.373324  0.369453  0.372771  178200
           data_KO['Date'] = pd.to_datetime(data_KO['Date'])
           data_KO.set_index('Date', inplace=True)
           data_PEP['Date'] = data_PEP.index
           data_PEP['Date'] = pd.to_datetime(data_PEP['Date'])
           data_PEP.set_index('Date', inplace=True)
       Closing Price: KO vs PEP
   import matplotlib.pyplot as plt
   # Create a new DataFrame with common dates
   common_dates=data_KO.index.intersection(data_PEP.index)
   data_combined= pd.DataFrame({
    'KO_Close': data_KO['Close'].loc[common_dates]['KO'],
    'PEP_Close': data_PEP['Close'].loc[common_dates]['PEP'] }
   )
   # Plot the closing prices
   plt.style.use('dark_background')
   plt.figure(figsize=(10, 6))
   plt.plot(data_combined.index, data_combined['KO_Close'],
   color='blue', linewidth=2, label='KO')
   plt.plot(data_combined.index, data_combined['PEP_Close'],
   color='red', linewidth=2, label='PEP')
   plt.xlabel('Date')
   plt.ylabel('Closing Price')
   plt.title('KO vs PEP')
   plt.legend()
   plt.show()
      Correlation: KO vs PEP
                                         Correlation between KO and PEP: 0.98
    # Calculate the correlation between KO and PEP
    correlation = data_combined['KO_Close'].corr(data_combined['PEP_Close'])
    print(f'Correlation between KO and PEP: {correlation:.2f}')
      Scatter Plot: KO vs PEP
    # Scatter plot between KO and PEP closing prices
    plt.figure(figsize=(10, 6))
    plt.scatter(data_combined['KO_Close'], data_combined['PEP_Close'],
    alpha=0.5, color='purple')
    plt.title('Scatter Plot of KO vs PEP Closing Prices')
    plt.xlabel('KO Closing Price')
    plt.ylabel('PEP Closing Price')
    # Add a 1:1 line for reference
    max_price= max(data_combined['KO_Close'].max(),
    data_combined['PEP_Close'].max())
    plt.plot([0, max_price], [0, max_price], color='red', linestyle='--',
    linewidth=1, label='1:1 Line')
    plt.show()
      Price Difference: KO vs PEP
  # Calculate the price difference
  price_difference = data_combined['KO_Close'] -
  data_combined['PEP_Close']
  # Calculate the mean of the price difference
  mean_difference = price_difference.mean()
  # Plotting the price difference
  plt.figure(figsize=(14, 7))
  plt.plot(price_difference.index, price_difference.values,
  label='Price Difference (KO - PEP)', color='blue')
  plt.axhline(mean_difference, color='red', linestyle='--',
  linewidth=1, label='Mean Difference') # Mean line
  plt.title('Price Difference between Coca-Cola (KO) and 
  PepsiCo (PEP)')
  plt.xlabel('Date')
  plt.ylabel('Price Difference')
  plt.legend()
  plt.grid()
  plt.show()
  Does this pair still valid nowadays? 
     Potential reasons
     • Fundamental Changes:
       • Company Performance: Changes in the operational performance, financial health, or strategic 
        direction of either KO or PEP can lead to divergence in their stock prices.
       • Market Positioning: If one company captures more market share or shifts its product strategy 
        significantly (e.g., focusing on health-conscious products), it may affect its stock price relative 
        to the other.
     • Economic Factors:
       • Economic downturns or inflation can affect consumer spending patterns, impacting KO and 
        PEP differently based on their product offerings and market strategies.
     • Investor Behavior:
       • Changes in investor sentiment towards either company (e.g., due to negative news, earnings 
        reports, or analyst downgrades) can lead to price movements that do not correspond with 
        historical patterns.
      Cointegration
   Cointegration
   • Most financial time series are not stationary nor mean reverting. 
   • However, we can create a portfolio of individual price series so that 
    the market value (or price) series of this portfolio is stationary. 
   • If we can find a stationary linear combination of several non-
    stationary price series, then these price series are called cointegrated. 
                 Definition and Formula
                 • Two time series  ÿ , Ā are cointegrated if there exists a constant β
                                                                    ���㕡          ���㕡
                     such that ā = ÿ 2βĀ is stationary
                                                 ���㕡             ���㕡              ���㕡
                 • Here β represents the cointegrating coefficient.
                 • In market practice, β is also called the hedge ratio suggesting the 
                     number of shares to long (short) Y for each share short (long) on X
     Statistical Tests for Cointegration
     • Cointegrated Augmented Dickey Fuller Test (CADF)
     • Steps: 
       • We first determine the optimal hedge ratio by running a linear regression 
        between two price series
       • Use this hedge ratio to form a portfolio
       • Then run an ADF test on this portfolio price series {ā = ÿ 2 βĀ }
                                        ���㕡 ���㕡  ���㕡
       • If the null hypothesis is rejected, we can conclude that ā is stationary, and 
        hence {Ā,ÿ}are cointegrated       ���㕡
      Python Example
                                    Cointegration test p-value: 0.9555567191241595
     import numpy as np
     import pandas as pd
     import statsmodels.api as sm
     import matplotlib.pyplot as plt
     import yfinance as yf
     start_date = '2020-01-01'
     end_date='2025-08-31'
     ko=yf.Ticker("KO").history(start=start_date, end=end_date)
     pep=yf.Ticker("PEP").history(start=start_date, end=end_date)
     # Price data 
     ko_close = ko['Close']
     pep_close= pep['Close']
     # Cointegration test 
     result = sm.tsa.stattools.coint(ko_close, pep_close)
     print('Cointegration test p-value:', result[1])
         Steps to Implement Pair Trading
         1.  Identify a pair of correlated assets.
         2.  Test for co-integration between the assets.
         3.  Calculate the spread and determine the historical mean and standard 
             deviation.
         4.  Generate trading signals when the spread deviates from the mean by a certain 
             threshold.
         5.  Execute trades: Go long on the undervalued asset and short on the overvalued 
             asset.
         6.  Close positions when the spread reverts to the mean.
  Market Entry/Exit based on the Spread’s Distance
    Key Components
    • Co-integration: A statistical property indicating a long-term 
     equilibrium relationship between two time series.
    • Spread: The difference in price between the two assets being traded.
    • Mean Reversion: The assumption that the spread will revert to its 
     historical average.
    Cointegration for multiple 
   variables (more than 2 assets)
                 Cointegrated Augmented Dickey Fuller Test
                 • Create a multiple linear regression model
                                                                   Ā =���㗽 +���㗽 ÿ +ï+���㗽 ÿ +���㔀
                                                                     ���㕡        0         1 1���㕡                    ���㕘   ���㕘���㕡       ���㕡
                 • Steps: 
                         • We first determine the optimal hedge ratios by running a multiple linear 
                             regression between the price series
                         • Use the hedge ratios {���㗽 } to form a portfolio
                                                                                 ÿ
                         • Then run an ADF test on this portfolio price series {ā = Ā 2 ���㗽 2 ���㗽 ÿ                                                                                             2
                             ï2���㗽 ÿ }                                                                                                           ���㕡          ���㕡         0           1 1���㕡
                                           ���㕘    ���㕘���㕡
                         • If null hypothesis is rejected, we can conclude that ā is stationary, and hence 
                             {Ā,ÿ ,…,ÿ }are cointegrated                                                                                        ���㕡
                                        1              ���㕘
     Limitation of above idea
     • If null hypothesis cannot be rejected, we only know that 
      {Ā,ÿ ,…,ÿ }cannot form a stationary time series. However, it is 
         1   ���㕘
      possible that a subset (eg. Ā,ÿ and ÿ ) can be cointegrated. The 
                          1    2
      CADF test doesn’t provide further insights about this. 
     • It only assumes a multiple linear relationship between the stocks, but 
      ignore other useful variables (eg. economic factors, technical 
      indicators, etc). 
           Vector Error Correction Model (VECM)
           • The concept of cointegration for multiple variables can be captured using the 
             Vector Error Correction Model (VECM) framework. 
           • General Framework:
                1. Vector Autoregressive Model (VAR):
                     Consider a vector of multiple non-stationary time series:             Ā
                                                                                            1���㕡
                                                                                            î
                                                                                   ���㖀 =
                                                                                     ���㖕
                                                                                           Ā
                                                                                            Ā���㕡
                     A VAR model can be expressed as:                        Where
                                                                             •   A: Matrix of coefficients.
                                          ���㖀 = ý���㖀       +þÿ +���㕁
                                            ���㖕      ���㖕2���㗏      ���㕡    ���㖕      •   ÿ : Exogenous variables (if any)
                                                                             •   ���㔇���㕡: Error term
                                                                                  ���㕡
      Vector Error Correction Model (VECM)
      • General Framework:
         2. Conintegration
            If the series are cointegrated, there exists a matrix ���㗽 such that:
                                    ���㕇
                                   ���㗽 Ā ~���㔼(0)
                                      ���㕡
             ���㔼(0) is a notation referring to a stationary time series without order differencing.
            This means that a linear combination of the series, given by ���㗽���㕇Ā is stationary
                                                      ���㕡
              Johansen Test for Cointegration
              • It tests for cointegration of more than 2 variables
              • Let’s generalize the discrete version of the ADF equation. 
                     • Ā are vectors representing multiple time series
                          ���㕡
                     • Λ, ý are matrices
                               ���㕘
                                                 ∆Ā =ΛĀ                  +Ā+ý∆Ā +ï+ý ∆Ā +ε
                                                     ���㕡           ���㕡21                   1      ���㕡21                    ���㕘     ���㕡2���㕘        ���㕡
              • Just in the univariate case, cointegration doesn’t exist if Λ=0
              • Denote the rank of Λ is r and the number of time series is n
                     • So testing whether Λ=0 will be equivalent to testing r=0
    Johansen Test for Cointegration
    • The number of independent portfolios that can be formed by various 
    linear combinations of the cointegrating price series is equal to r. The 
    Johansen test will calculate r in two different ways, both based on the 
    eigenvector decomposition of Λ.
     1. Trace Statistic
     2. Maximum Eigenvalue Statistic
     Johansen Test for Cointegration
     • Meaning of ���㕟:
       • It represents the number of cointegrating vectors among the time series 
        being analyzed.
         • If ���㕟=0: There are no cointegrating relationships; the time series are not stationary in any 
          linear combination.
         • If ���㕟>0: There are ���㕟 cointegrating relationships, indicating that some linear combinations 
          of the time series are stationary, suggesting long-run equilibrium relationships among 
          them.
     • As a useful by-product, the eigenvectors found can be used as our hedge ratios 
      for the individual price series to form a stationary portfolio.
             Johansen Test for Cointegration
                    1. Trace Statistic
                          • Tests the null hypothesis that the number of cointegrating vectors is less than or equal to ���㕟
                                                                                       ���㕘
                                                                                                             
                                                    ÿ���㕅            ���㕟    =2���㕇   log(12λ )
                                                         ���㕡���㕟���㕎���㕐���㕒                                           ÿ
                                                                                   ÿ=���㕟+1
                                    where
                                    • T: Sample size
                                    • k: Number of variables in the system
                                          
                                    • λ : The estimated eigenvalues from the matrix of Λ
                                          ÿ
        Johansen Test for Cointegration
           2. Maximum Eigenvalue Statistic
               • Tests the null hypothesis that there is exactly ���㕟 cointegrating vectors 
                 against the alternative of ���㕟+1
                                                            
                              ÿ���㕅    (���㕟) = 2���㕇 ���㕙���㕜���㕔(1 2 λ     )
                                 ÿ���㕎���㕥                       ���㕟+1
               • It focuses specifically on the (���㕟 + 1)���㕡/ eigenvalue
    Python Example
    • Determine whether the <Magnificent 7=stocks are cointegrated using 
     the daily closing price from 2020 to 2025
      • AAPL
      • AMZN
      • NVDA
      • MSFT
      • TSLA
      • META
      • GOOGL
        import numpy as np
        import pandas as pd                                                                                   Stock prices data:
        import yfinance as yf                                                                                  Ticker           AAPL       AMZN      GOOGL        META        MSFT      NVDA       
                 Python Example                                                                               TSLA
        from statsmodels.tsa.vector_ar.vecm import coint_johansen
                                                                                                              Date
        # Define the stock symbols                                                                            2020-01-02  72.400505  94.900497  67.920815  208.324753  152.505692  5.971076  
        stocks = ['AAPL', 'AMZN', 'NVDA', 'MSFT', 'TSLA', 'META', 'GOOGL']                                    28.684000
                                                                                                              2020-01-03  71.696617  93.748497  67.565483  207.222488  150.606720  5.875504  
        # Download the stock data                                                                             29.534000
                                                                                                              2020-01-06  72.267921  95.143997  69.366386  211.125244  150.995987  5.900144  
        df = yf.download(stocks, start='2020-01-01', end='2025-12-31')['Close']                               30.102667
                                                                                                              2020-01-07  71.928047  95.343002  69.232391  211.582031  149.619278  5.971575  
        # Display the first few rows of the data                                                              31.270666
        print("Stock prices data:\n", df.head())                                                              2020-01-08  73.085098  94.598503  69.725174  213.727051  152.002472  5.982776  
                                                                                                              32.809334
        # Perform the Johansen test                                                                           Eigenvalues:
        result = coint_johansen(df, det_order=0, k_ar_diff=1)                                                  [0.02747738 0.02070775 0.011672   0.00785587 0.00530318 0.00366997
                                                                                                               0.00042318]
        # Check the results                                                                                   Critical values (trace):
        print("\nEigenvalues:\n", result.eig)                                                                  [[120.3673 125.6185 135.9825]
        print("\nCritical values (trace):\n", result.cvt)                                                      [ 91.109   95.7542 104.9637]
        print("\nCritical values (max eigenvalue):\n", result.cvm)                                             [ 65.8202  69.8189  77.8202]
                                                                                                               [ 44.4929  47.8545  54.6815]
        # Determine the rank based on the trace statistic                                                      [ 27.0669  29.7961  35.4628]
        trace_stat = result.lr1                                                                                [ 13.4294  15.4943  19.9349]                               (90%, 95%, 99%) 
        critical_values = result.cvt[:, 1] # 5% critical values                                                [  2.7055   3.8415   6.6349]]                              confidence level
        rank = np.sum(trace_stat > critical_values)                                                           Critical values (max eigenvalue):
                                                                                                               [[43.2947 46.2299 52.3069]
                                                                                                               [37.2786 40.0763 45.8662]
        print("\ntrace_stat=",trace_stat)                                                                      [31.2379 33.8777 39.3693]
        print("\nNumber of cointegration relationships (rank):", rank)                                         [25.1236 27.5858 32.7172]
                                                                                                               [18.8928 21.1314 25.865 ]
        # Determine the rank based on the maximum eigenvalue statistic                                         [12.2971 14.2639 18.52  ]
        max_eigen_stat= result.lr2                                                                             [ 2.7055  3.8415  6.6349]]
        critical_values = result.cvm[:, 1] # 5% critical values                                               trace_stat= [117.13703785  75.20481929  43.71245014  26.0427652   14.1730027
                                                                                                                 6.17048494   0.63701792]
        rank = np.sum(max_eigen_stat > critical_values)                                                       Number of cointegration relationships (rank): 0
        print("\nmax_eigen_stat=", max_eigen_stat)                                                            max_eigen_stat= [41.93221855 31.49236916 17.66968493 11.8697625   8.00251776  
        print("\nNumber of cointegration relationships (rank):", rank)                                        5.53346703  0.63701792]
   Reference: https://www.statsmodels.org/dev/generated/statsmodels.tsa.vector_ar.vecm.coint_johansen.html    Number of cointegration relationships (rank): 0
   Cointegration vs Correlation
     Correlation
     • Measures the strength and direction of a linear relationship between 2 variables.
     • The value is ranging between -1 and 1
       • 1: Perfect positive linear relationship
       • -1: Perfect negative linear relationship
       • 0: No linear relationship
     • Limitation: Does not imply a long-term equilibrium relationship.
   Cointegration
   • Indicates a long-term equilibrium relationship between 2 or more 
    time series, even if they are non-stationary individually.
   • If two series are cointegrated, they will move together over time, 
    despite short-term deviations.
     Key Differences
     • Nature:
       • Correlation is about the short-term relationship.
       • Cointegration focuses on the long-term relationship.
     • Application:
       • Use correlation for diversification.
       • Use cointegration for identifying tradable pairs.
   Implement a Cointegration
       Strategy
     Cointegration Strategy
     • Let’s create a cointegration strategy for banking sector stocks
       • 00005HK (i.e. HSBC)
       • 00011HK (i.e. Hang Seng Bank)
       • 00939HK (i.e. China Construction Bank)
       • 02388HK (i.e. Bank of China Hong Kong)
       • 03968HK (i.e. China Merchants Bank)
     • BacktestSetup/ Assumptions
       • Instrument: 00005HK, 00011HK, 00939HK, 02388HK, 03968HK
       • Backtest period: 2021.01 – 2024.12
       • Initial capital: HK$100k
       • Short-selling are possible
       • We can trade odd lot
  Market Entry/Exit based on the Spread’s Distance
     Cointegration Script (1)
      from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest
      import pandas as pd
      import numpy as np
      import statsmodels.api as sm
      from statsmodels.tsa.stattools import coint
      class AlgoEvent:
        def __init__(self):
         self.order_size = 100
         self.entry_threshold = 1
         self.pair_id = 0
         self.contractSize = {}
        def start(self, mEvt):
         self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)
         for instrument in mEvt['subscribeList']:
          self.contractSize[instrument] = self.evt.getContractSpec(instrument)["contractSize"]
         self.evt.start()
               Cointegration Script (2)
       def on_bulkdatafeed(self, isSync, bd, ab):
          if not isSync:
            return
          # get historical data
          instruments = list(bd)
          data = {}
          for instrument in instruments:
            obs=self.evt.getHistoricalBar(contract={"instrument":instrument}, numOfBar=100, interval="D")
            data[instrument] = {t: obs[t]['c'] for t in obs}
          df = pd.DataFrame(data)
          df = df.fillna(method='ffill')
          df = df.dropna()
          self.evt.consoleLog("df=",df)
          # cointegration matrix and pair
          cointegrated_pairs, coint_df = find_cointegration_pair(instruments, df, threshold=0.05)
          self.evt.consoleLog("coint_df=",coint_df)                                                      def find_cointegration_pair(instruments, df, threshold=0.05):
          self.evt.consoleLog("\nCointegrated Pairs:")                                                     n = len(instruments)
          for pair in cointegrated_pairs:                                                                  coint_matrix = np.zeros((n, n))
            self.evt.consoleLog(pair)                                                                      for i in range(n):
                                                                                                              for j in range(i + 1, n):
                                                                                                                p_value = coint(df[instruments[i]], df[instruments[j]])[1]
                                                                                                                coint_matrix[i, j] = p_value
                                                                                                           cointegrated_pairs = [(instruments[i], instruments[j]) for i in range(n) for j in
                                                                                                         range(i + 1, n) if coint_matrix[i, j] < threshold]
                                                                                                           return cointegrated_pairs, coint_matrix
                  Cointegration Script (3)
                   # Find hedge ratios and trading signals
                   for stock1, stock2 in cointegrated_pairs:
                      self.pair_id += 1
                      # Fit linear regression to find hedge ratio
                      Y=df[stock1]
                      X=df[stock2]
                      X=sm.add_constant(X)
                      model = sm.OLS(Y, X).fit()
                      hedge_ratio = model.params[1]                                                                                   # long stock1 and short stock2
                      # Calculate the spread and Z-score                                                                              elif z_score[-1] < -1*self.entry_threshold:
                      spread = Y - hedge_ratio * df[stock2]                                                                             order1 = AlgoAPIUtil.OrderObject(
                      z_score = (spread - spread.mean()) / spread.std()                                                                    instrument = stock1,
                      self.evt.consoleLog('debug ... z_score=',z_score[-1])                                                                openclose = 'open',
                      # short stock1 and long stock2                                                                                       buysell = 1, #sell
                      if z_score[-1] > self.entry_threshold:                                                                               ordertype = 0, #market order
                        order1 = AlgoAPIUtil.OrderObject(                                                                                  volume=1/self.contractSize[stock1]*self.order_size, #1 share of stock 1
                           instrument = stock1,                                                                                            orderRef= self.pair_id
                           openclose = 'open',                                                                                          )
                           buysell = -1, #sell                                                                                          self.evt.sendOrder(order1)
                           ordertype = 0, #market order                                                                                 order2 = AlgoAPIUtil.OrderObject(
                           volume = 1/self.contractSize[stock1]*self.order_size, #1 share of stock1                                        instrument = stock2,
                           orderRef = self.pair_id                                                                                         openclose = 'open',
                        )                                                                                                                  buysell = -1 if hedge_ratio>0 else 1,
                        self.evt.sendOrder(order1)                                                                                         ordertype = 0, #market order
                        order2 = AlgoAPIUtil.OrderObject(                                                                                  volume=hedge_ratio/self.contractSize[stock2]*self.order_size,
                           instrument = stock2,                                                                                            orderRef= self.pair_id
                           openclose = 'open',                                                                                          )
                           buysell = 1 if hedge_ratio>0 else -1,                                                                        self.evt.sendOrder(order2)
                           ordertype = 0, #market order
                           volume = hedge_ratio/self.contractSize[stock2]*self.order_size,
                           orderRef = self.pair_id
                        )
                        self.evt.sendOrder(order2)
       Cointegration Script (4)
          # check order exit
          pairs = {}
          pos, osOrder, pendOrder = self.evt.getSystemOrders()
          for tradeID in osOrder:
           order = osOrder[tradeID]
           instrument = order['instrument']
           pair_id = order['orderRef']
           buysell = order['buysell']
           volume=order['Volume']
           openprice = order['openprice']
           pl = order[8pl']
           mktPrice = bd[instrument]['bidPrice'] if buysell==1 else bd[instrument]['askPrice']
           if pair_id not in pairs:
            pairs[pair_id] = {"entry_spread":0, "current_spread":0, "count":0, “pl":0}
           pairs[pair_id]["count"]+=1
           pairs[pair_id][“pl"]+= pl
           pairs[pair_id]["entry_spread"]+= -1*buysell*volume*openprice
           pairs[pair_id]["current_spread"]+= buysell*volume*mktPrice
          for pair_id in pairs:
           if pairs[pair_id]["count"]==2 and ( \
            (pairs[pair_id]["entry_spread"]>0 and pairs[pair_id]["current_spread"]<=0) or \
            (pairs[pair_id]["entry_spread"]<0 and pairs[pair_id]["current_spread"]>=0) \
           ) and pairs[pair_id][“pl"]>0:
            order = AlgoAPIUtil.OrderObject(
             openclose = 'close',
             orderRef = pair_id
            )
            self.evt.sendOrder(order)
                                                                                                                                      defon_bulkdatafeed(self,isSync,bd,ab):
                                                                                                                                        if not isSync:
                                                                                                                                          return
                                                                                                                                        # get historical data
                                                                                                                                        instruments= list(bd)
                                                                                                                                        data= {}
                                                                                                                                        for instrumentin instruments:
                                                                                                                                          obs=self.evt.getHistoricalBar(contract={"instrument":instrument},numOfBar=100,interval="D")
                                                                                                                                          data[instrument]= {t: obs[t]['c']for t in obs}
                                                                                                                                        df = pd.DataFrame(data)
                           Full Script                                                                                                  df = df.fillna(method='ffill')
                                                                                                                                        df = df.dropna()
                                                                                                                                        self.evt.consoleLog("df=",df)
                                                                                                                                        # cointegrationmatrix and pair
                                                                                                                                        cointegrated_pairs, coint_df= find_cointegration_pair(instruments,df, threshold=0.05)
                                                                                                                                        self.evt.consoleLog("coint_df=",coint_df)                                                                 # check order exit
                                                                                                                                        self.evt.consoleLog("\nCointegratedPairs:")                                                               pairs = {}
                                                                                                                                        for pair in cointegrated_pairs:                                                                           pos, osOrder, pendOrder = self.evt.getSystemOrders()
        from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest                                                                                 self.evt.consoleLog(pair)                                                                               for tradeID in osOrder:
        import pandas as pd                                                                                                             # Find hedge ratios and trading signals                                                                      order = osOrder[tradeID]
        import numpy as np                                                                                                              for stock1,stock2 in cointegrated_pairs:                                                                     instrument = order['instrument']
        import statsmodels.api as sm                                                                                                      self.pair_id += 1                                                                                          pair_id = order['orderRef']
        from statsmodels.tsa.stattools                                                                                                    # Fit linear regression to find hedge ratio                                                                buysell = order['buysell']
        import coint                                                                                                                      Y=df[stock1]
                                                                                                                                          X=df[stock2]                                                                                               volume = order['Volume']
                                                                                                                                          X=sm.add_constant(X)                                                                                       openprice = order['openprice']
        def find_cointegration_pair(instruments, df, threshold=0.05):                                                                     model=sm.OLS(Y,X).fit()                                                                                    mktPrice = bd[instrument]['bidPrice'] if buysell==1 else bd[instrument]['askPrice']
           n = len(instruments)                                                                                                           hedge_ratio= model.params[1]                                                                               if pair_id not in pairs:
           coint_matrix = np.zeros((n, n))                                                                                                # Calculate the spread and Z-score                                                                            pairs[pair_id] = {"entry_spread":0, "current_spread":0, "count":0, “pl":0}
           for i in range(n):                                                                                                             spread= Y- hedge_ratio* df[stock2]                                                                         pairs[pair_id]["count"]+=1
              for j in range(i + 1, n):                                                                                                   z_score = (spread- spread.mean()) / spread.std()                                                           pairs[pair_id][“pl"]+=pl
                 p_value = coint(df[instruments[i]], df[instruments[j]])[1]                                                               self.evt.consoleLog('debug ... z_score=',z_score[-1])                                                      pairs[pair_id]["entry_spread"]+= -1*buysell*volume*openprice
                 coint_matrix[i, j] = p_value                                                                                             # short stock1 and long stock2                                                                             pairs[pair_id]["current_spread"]+= buysell*volume*mktPrice
           cointegrated_pairs = [(instruments[i], instruments[j]) for i in range(n) for j in range(i + 1, n) if                           if z_score[-1] > self.entry_threshold:                                                                  for pair_id in pairs:
        coint_matrix[i, j] < threshold]                                                                                                     order1=AlgoAPIUtil.OrderObject(                                                                          if pairs[pair_id]["count"]==2 and ( \
           return cointegrated_pairs, coint_matrix                                                                                            instrument= stock1,
                                                                                                                                              openclose='open',                                                                                         (pairs[pair_id]["entry_spread"]>0 and pairs[pair_id]["current_spread"]<=0) or \
                                                                                                                                              buysell= -1, #sell                                                                                        (pairs[pair_id]["entry_spread"]<0 and pairs[pair_id]["current_spread"]>=0) \
        class AlgoEvent:                                                                                                                      ordertype= 0, #market order                                                                            ) and pairs[pair_id][“pl"] >0:
           def __init__(self):                                                                                                                volume=1/self.contractSize[stock1]*self.order_size,#1 share of stock1                                     order = AlgoAPIUtil.OrderObject(
              self.order_size = 100                                                                                                           orderRef= self.pair_id
                                                                                                                                            )                                                                                                              openclose = 'close',
              self.entry_threshold = 1                                                                                                      self.evt.sendOrder(order1)                                                                                     orderRef = pair_id
              self.pair_id = 0                                                                                                              order2=AlgoAPIUtil.OrderObject(                                                                             )
              self.contractSize = {}                                                                                                          instrument= stock2,                                                                                       self.evt.sendOrder(order)
                                                                                                                                              openclose='open',
                                                                                                                                              buysell= 1 if hedge_ratio>0else -1,
           def start(self, mEvt):                                                                                                             ordertype= 0, #market order
              self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)                                                                          volume=hedge_ratio/self.contractSize[stock2]*self.order_size,
              for instrument in mEvt['subscribeList']:                                                                                        orderRef= self.pair_id
                                                                                                                                            )
                 self.contractSize[instrument] = self.evt.getContractSpec(instrument)["contractSize"]                                       self.evt.sendOrder(order2)
              self.evt.start()                                                                                                            # long stock1 and short stock2
                                                                                                                                          elif z_score[-1]< -1*self.entry_threshold:
                                                                                                                                            order1=AlgoAPIUtil.OrderObject(
                                                                                                                                              instrument= stock1,
                                                                                                                                              openclose='open',
                                                                                                                                              buysell= 1, #sell
                                                                                                                                              ordertype= 0, #market order
                                                                                                                                              volume=1/self.contractSize[stock1]*self.order_size,#1 share of stock 1
                                                                                                                                              orderRef= self.pair_id
                                                                                                                                            )
                                                                                                                                            self.evt.sendOrder(order1)
                                                                                                                                            order2=AlgoAPIUtil.OrderObject(
                                                                                                                                              instrument= stock2,
                                                                                                                                              openclose='open',
                                                                                                                                              buysell= -1 if hedge_ratio>0else 1,
                                                                                                                                              ordertype= 0, #market order
                                                                                                                                              volume=hedge_ratio/self.contractSize[stock2]*self.order_size,
                                                                                                                                              orderRef= self.pair_id
                                                                                                                                            )
                                                                                                                                            self.evt.sendOrder(order2)
  Backtest Result
  Backtest Result
    Real Market Challenges
    • Model Risk: Incorrect assumptions or overfitting can lead to poor performance.
    • Execution Risk: Slippage and latency can affect the profitability of trades.
    • Market Risk: Unforeseen market events can disrupt statistical relationships.
    • Regulatory Risk: Changes in market regulations can impact strategy viability. (eg. 
     short selling, leverage requirement, etc)
    Wrap Up
    • Statistical arbitrage is a popular strategy used by many quant trading 
     firms that effectively make profits over years. 
    • From statistics point of view, it has a high probability of winning
    • To run this strategy effectively, 
     • Select the group of assets that are cointegrated
     • Try different observation period for cointegration analysis
     • Choose a distance measure to determine optimal entry and exit points
     • Data transformation of the original price series may increase significance of 
      cointegration
     • May worth to rebalance portfolio due to hedge ratio changes
    Overview of FX Market
    Foreign Exchange Market
    • The Foreign Exchange market, is also called FX or forex, is the largest 
     asset class in terms of daily trading volume
    • It has the following properties:
      1. Decentralized market
      2. Long Trading Hour
      3. High Turnover
      4. Low Spread
      5. Rollover Swap
      6. High Leverage
     Centralized vs Decentralized Market
     • FX market operates by the market itself. There is no single entity responsible for 
      central clearing. 
     • More price transparency and less market manipulation
    Long Trading Hour
    • FX market operates in 24*5.5
    • It starts trading from 21:00 GMT in Sydney on Sunday to 20:00 GMT 
     in New York on Friday
      • i.e. Monday 5am (HKT) to Saturday 4am (HKT)
    • More trading opportunities 
    • Less market gap and price jumps
    High Turnover
    • Global equity market: $3.93 
     trillion
    • Forex: $6.6 trillion
    Low Spread
    • For EURUSD, the percentage spread is 
        (1.11376 –1.11364)/((1.11376+1.11364)/2 ) = 0.0108%
    Low Spread
    • For TSLA, the percentage spread is 
     (428.05 – 426.73)/((428.05 + 426.73)/2 ) = 0.031%
    Rollover Swap
    • FX trading involves buying 1 currency and selling 
     another currency
    • You will receive interest from the currency you buy; 
     and pay interest for the currency you sell
    • If the net interest is positive, you will have positive 
     carry for holding your position overnight. 
    • The net interest rate is called <overnight swap=, or 
     simply <swap=
    • The swap rates are updating every day based on the 
     funding costs the FX brokers need to maintain its risks 
     and operations
    • For the same instrument, the swap rates can be 
     different for different FX brokers
    Swap Schedule
    • The swap payment, if any, will be 
     settled in your broker account every 
     day.  
    • To handle market closure in 
     weekends, the swap fee will be 
     calculated 3 times on Wednesday
    • That means if you have overnight 
     position on Wednesday, you will 
     receive/pay 3 times of the swap fee. 
   High Leverage
   • Compared to stock market, FX is less regulated due to the 
    decentralization property
   • Many FX brokers can allow users to trade at 100x leverage. Some 
    even supports 5000x
   • For a Hong Kong regulated FX broker, investors can only trade up to 5x 
    leverage
   Market Quotation
   • In FX market, it is usually quoted as XXXYYY
   • It means that for 1 unit of XXX, how much YYY can you get?
   Market Quotation
   • Some brokers may quote in the form of YYY/XXX 
   • It means that for 1 unit of XXX, how much YYY can you get?
    Whichcurrency do I buy?
    • For stock, a price quotation is $xxx / 1 share of yyy
    • For currency, a price is quoted in the amount of currency XXX / 1 unit 
     of currency YYY. 
      • 7.79 simple means 7.79 HKD / 1 USD
      Mostly Traded Currencies
                                               Sum of column = 200%
              Reference: https://en.wikipedia.org/wiki/Template:Most_traded_currencies
    Trading USDHKD Peg
                  Sure Win???
   What is a Currency Peg?
   • Definition: A currency peg is a policy where a country's currency 
    value is tied or fixed to another major currency.
   • Purpose: To stabilize the exchange rate and provide predictability for 
    international trade.
    The USDHKD Peg
    • In 1983, the Hong Kong Gov introduced to peg the exchange rate with 
     USD.
    • The Hong Kong Dollar (HKD) is pegged to the US Dollar (USD) at 
     approximately 7.8 HKD to 1 USD.
    • The Hong Kong Monetary Authority (HKMA) maintains this peg 
     through market interventions.
          Pros & Cons of USDHKD peg
            Pros                                               Cons
            Stability: Reduces exchange rate volatility for    Loss of Monetary Policy Autonomy: HKMA 
            businesses and investors.                          cannot freely adjust interest rates.
            Trade Facilitation: Simplifies international       Vulnerability to External Shocks: Economic 
            trade with the US                                  issues in the US can impact Hong Kong.
            Inflation Control: Helps maintain low inflation  Speculative Attacks: Potential for market 
            rates in Hong Kong                                 speculation against the peg.
    The USDHKD Peg
    • Allows fluctuation between 7.75 and 7.85
    • If the exchange rate is approaching 7.75, that means HKD is getting 
     strong (or USD is weak). HKMA will sell HKD and buy USD in the 
     market
    • If the exchange rate is approaching 7.85, that means HKD is getting 
     weak (or USD is strong). HKMA will buy HKD and sell USD in the 
     market
    How retail investors can capture the opportunities?
    • You want to buy around 7.75 and sell around 7.85 to capture the 0.1 
    profits
    • Suppose you have 1m HKD at the beginning, 
     • At 7.75, you should sell all HKD to get 1m/7.75 = 129,032 USD
     • At 7.85, you then sell all USD to get 129,032*7.85 = 1,012,903 HKD
     Is the USDHKD peg strategy safe?
     • Can international hedge funds still attack 
      the peg mechanism just like Asia Financial 
      Crisis in 1997?
     • What if HKMA run out of capital to 
      interfere the market?
                                           George Soros 
     Is the USDHKD peg strategy safe?
     • If HKD is depreciating toward 7.85, that means the market is dumping HKD. 
     • HKMA has sufficient reserve to buy back all circulating HKD in the market. 
  Reference: https://www.hkma.gov.hk/eng/news-and-media/press-releases/2026/02/20260206-3/
   Is the USDHKD peg strategy safe?
   • If the exchange rate is moving toward 7.75 (i.e. HKD is appreciating), 
    that means the market has growing demand for HKD. 
   • HKMA can easily print more HKD
    Simple USD/HKD trading strategy
    Trading Logic:
      • Get the current market price of USD/HKD
      • Entry logic:
        • If current position is zero, 
          • If market price is less than 7.76, open a buy order with all available capital
          • If market price is great than 7.84, open a sell order with all available capital
      • Exit logic:
        • If current position is non-zero, 
          • If it previously buy at 7.76 but now rebound to 7.8 or above, close order
          • If it previously sell at 7.84 but now reverse to 7.8 or below, close order
    Backtest
    • Settings
      • Instrument: USDHKD
      • Period: 2020-01 to 2024-12
      • Enable short-sell: YES
      • Leverage: 1x
      • Data Interval: 1-day
      • Transaction Cost: 0
      • Initial Capital: 100k HKD
     Backtest
     • Define variables at initialization
       class AlgoEvent:
         def __init__(self):
          self.thre_b = 7.76
          self.thre_s = 7.84
          self.thre_exit = 7.8
          self.timer = datetime(1970,1,1)
     Backtest
     • Get contract size for USDHKD
     • In forex market, 1 lot = 100,000 unit of settle currency (i.e. HKD)
       def start(self, mEvt):
         self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)
         # get contract size of USDHKD
         instrument = mEvt['subscribeList'][0]
         spec = self.evt.getContractSpec(instrument)
         self.contractSize = spec["contractSize"]
         self.evt.start()
     Backtest
     • Get current position
       def on_marketdatafeed(self, md, ab):
        if md.timestamp>=self.timer+timedelta(hours=24):
         self.timer = md.timestamp
         # get current position
         pos, osOrder, pendOrder = self.evt.getSystemOrders()
         position = pos[md.instrument]["netVolume"]
                           if position==0:
     Backtest               # calculate the max lot can trade based on current balance
                            lotSize = ab['availableBalance']/self.contractSize/md.askPrice
                            lotSize = round( 0.01*floor(100*lotSize), 2)
     • Order Entry          if md.askPrice<=self.thre_b:
                             order = AlgoAPIUtil.OrderObject(
                              instrument = md.instrument,
                              buysell = 1,
                              ordertype = 0, #market order
                              openclose = 'open9,
                              volume = lotSize,
                              orderRef = 'Long9
                             )
                             self.evt.sendOrder(order)
                            elif md.bidPrice>=self.thre_s:
                             order = AlgoAPIUtil.OrderObject(
                              instrument = md.instrument,
                              buysell = -1,
                              ordertype = 0, #market order
                              openclose = 'open9,
                              volume = lotSize,
                              orderRef = 'Short9
                             )
                             self.evt.sendOrder(order)
     Backtest
     • Order Exit
                        # order exit
                        elif position>0 and md.bidPrice>=self.thre_exit:
                          order = AlgoAPIUtil.OrderObject(
                           openclose = 'close9,
                           orderRef = 'Long9
                          )
                          self.evt.sendOrder(order)
                        elif position<0 and md.askPrice<=self.thre_exit:
                          order = AlgoAPIUtil.OrderObject(
                           openclose = 'close9,
                           orderRef = 'Short9
                          )
                          self.evt.sendOrder(order)
                                                                 def on_marketdatafeed(self, md, ab):
                                                                  if md.timestamp>=self.timer+timedelta(hours=24):
                                                                   self.timer = md.timestamp
                                                                   # get current position
         Backtest                                                  pos, osOrder, pendOrder = self.evt.getSystemOrders()
                                                                   position = pos[md.instrument]["netVolume"] # order entry
                                                                   if position==0:
                                                                    # calculate the max lot can trade based on current balance
                                                                    lotSize = ab['availableBalance']/self.contractSize/md.askPrice
         • Full script                                              lotSize = round( 0.01*floor(100*lotSize), 2)
                                                                    if md.askPrice<=self.thre_b:
                                                                     order = AlgoAPIUtil.OrderObject(
                                                                      instrument = md.instrument,
                                                                      buysell = 1,
           from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest          ordertype = 0, #market order
           from datetime import datetime, timedelta                   openclose= 'open9,
           from math import floor                                     volume=lotSize,
                                                                      orderRef = 'Long9
                                                                     )
           class AlgoEvent:                                          self.evt.sendOrder(order)
            def __init__(self):                                     elif md.bidPrice>=self.thre_s:
             self.thre_b = 7.76                                      order = AlgoAPIUtil.OrderObject(
             self.thre_s = 7.84                                       instrument = md.instrument,
             self.thre_exit = 7.8                                     buysell = -1,
             self.timer = datetime(1970,1,1)                          ordertype = 0, #market order
                                                                      openclose= 'open9,
            def start(self, mEvt):                                    volume=lotSize,
                                                                      orderRef = 'Short9
             self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)  )
                                                                     self.evt.sendOrder(order)
             # get contract size of USDHKD                         # order exit
             instrument = mEvt['subscribeList'][0]                 elif position>0 and md.bidPrice>=self.thre_exit:
             spec = self.evt.getContractSpec(instrument)            order = AlgoAPIUtil.OrderObject(
             self.contractSize = spec["contractSize"]                openclose= 'close9,
             self.evt.start()                                        orderRef = 'Long9
                                                                    )
                                                                    self.evt.sendOrder(order)
                                                                   elif position<0 and md.askPrice<=self.thre_exit:
                                                                    order = AlgoAPIUtil.OrderObject(
                                                                     openclose= 'close9,
                                                                     orderRef = 'Short9
                                                                    )
                                                                    self.evt.sendOrder(order)
  Backtest Results
  Return is too small
    Some ways to improve the strategy
    • Earlier entry so that it can trade more
    • Exit earlier such that the holding period can be shorter
    • Split the capital and trade at a different price level, eg.
      • 0.1 lot at 7.77
      • 0.2 lot at 7.76
      • 0.3 lot at 7.75
    • Engage in leverage trading
      Is it executable in the market?
      • Max profit = 7.85 / 7.75 – 1 = 1.29%
      • The percentage spread = 2*(7.8246 – 7.7568)/(7.8246 + 7.7568) = 0.87% 
  Reference: https://www.hsbc.com.hk/investments/products/foreign-exchange/currency-rate/
   How about other CFD brokers?
   • The percentage spread = 2*(7.79213 - 7.78938)/(7.8938 + 7.79213) = 
    0.035% 
    High Swap Fee
    • For long position, we are paying 
     9.3/100000 = 0.0093% every day
      • So 1.29% / 0.0093% = 139 days
    • For short position, we are paying 0.0194% 
     every day
      • So 1.29% / 0.0194% = 66 days
         Wrap Up
         • Without leverage, the maximum profit you can get is 7.85/7.75 – 1 = 1.29%
         • It usually takes a few months for the price to rebound.
         • For Spot market, 
              • Due to physical cash exchange, no leverage involved.                        Not profitable
              • The bid-ask spread are also too wide for currency exchanges. 
         • For CFD market, 
              • We can take leverage 
              • Acceptable bid-ask spread                                Not as profitable as expected
              • However overnight swap fee is too high
   Final Conclusion
   • This strategy has a high winning rate.
   • But it requires a good market timing for entry 
    and exit. 
   • Should keep your holding time short, preferably 
    within a few days.
   • No free lunch in the world!!!
    Key Takeaways
    • Statistical arbitrage is a type of strategy commonly used by many hedge funds 
     and investment banks. 
    • Co-integration is used to identify the basket of suitable instruments that can 
     construct a stationary time series
    • Correlation measures short terms relation while cointegration for long terms
    • FX Market has its unique properties and features
    • Discuss the risks and opportunities of the USD-HKD arbitrage 

</PLAIN_TEXT>

可用图片：
<IMAGE_HINTS>

</IMAGE_HINTS>

请直接输出 JSON，不要加解释。
