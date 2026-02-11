# NuxLearn - Linear Models

Linear regression and logistic regression implementations.

## Features
- Linear Regression (OLS, Ridge, Lasso)
- Logistic Regression (binary & multiclass)
- Regularization support
- Cross-validation

## API
```cpp
#include <nux_learn/supervised/linear/regression.h>

LinearRegression model;
model.Fit(X_train, y_train);
auto predictions = model.Predict(X_test);
double r2 = model.Score(X_test, y_test);
```

## Dependencies
- NuxArray (tensor operations)
- NuxSafe (validation)

## Build
```bash
cd supervised/linear
mkdir build && cd build
cmake ..
make
```
