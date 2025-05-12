# About this Dashboard WebApp.

- Hello my name is Sohail, this is a dashboad app that is solely coded on Rust Fullstack
  - Backend: Tokio, Axum, Hyper, Tower, Redis
  - Frontend: Dioxus


## Why Rust?

- Why rust you may ask, when there are many matured frameworks that are available both for backend and frontends, the answer lies in the 
memory safety feature of rust along with performance

- Why memory safety is important? because most of the  critical vulnerabilities catalogued by [MITRE CVE](https://cve.mitre.org/cgi-bin/cvekey.cgi?keyword=memory+leak) and [CISA KEV](https://www.cisa.gov/known-exploited-vulnerabilities-catalog?search_api_fulltext=memory&field_date_added_wrapper=all&field_cve=&sort_by=field_date_added&items_per_page=20&url=) are memory-related—buffer overflows, use-after-free, and out-of-bounds accesses—making up the bulk of high-severity bugs.

- Rust is your atypical language that took world by storm with its memory safety features that rely on borrowing and owning data rether than relying on a garbage collectors like java or manually dropping memory like C

- It offers near-C performance via zero-cost abstractions and direct control over memory allocation, making it ideal for system-level tasks

- Rust’s fearless concurrency model prevents data races at compile time, enabling safe multi-threaded applications without runtime overhead 
insights.sei.cmu.edu

- With Cargo and crates.io, Rust provides a streamlined package manager and ecosystem, simplifying dependency management and project builds 

- Strong tooling—like Clippy for linting and Rustfmt for consistent formatting—ensures code quality and developer productivity across the Rust ecosystem


# Contents of the DashApp

## Observations and Experiments carried:

➼ Fitting tests were done on all the independent variables of each dataset.

➼ Top 10 Best Fits were plotted with interactive graphs.

➼ Inferences were drawn on each variable and the strength of the fit and fit was weighed for further explanations.

➼ Machine Learning was implemented for relevant datasets.

➼ Classifications and Regression Observations were carried out for 
 - Mushroomn Classification
 - Time Series analysis for kfc-stock
 - Covid-19 effects on different states on India

➼ Different ML algorithms were used ranging from simple Linear Regression to complex Gradient Boosting bagging algorithms and Neural Nets.

➼ For every dataset an index page has been provided for feature descrptions, experimentation methods, data collection strategies, cleaning processes etc.,


## Steps Followed

### Data Collection
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⬇
### Data Preprocessing
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⬇
### Model Training
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⬇
### Model Evaluation
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⬇
### Results Interpretation