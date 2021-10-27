#include<math.h>

inline float mul_add(float a, float b, float c) {
    return a * b + c;
}

inline float from_bits(unsigned x) {
    union {
        float f;
        unsigned x;
    } u;
    u.x = x;
    return u.f;
}

typedef float f32;

f32 x_sin(f32 x) {
  float x = x * (1.0 / (M_PI * 2.0));
  float x = x - round(x);
  return mul_add(mul_add(mul_add(mul_add(mul_add(mul_add(mul_add((-from_bits(1058770289)), x * x, from_bits(1081164794)), x * x, -from_bits(1097945697)), x * x, from_bits(1109932662)), x * x, -from_bits(1117350231)), x * x, from_bits(1117992419)), x * x, -from_bits(1109745127)), x * x, from_bits(1086918619)) * x;
}

f32 x_cos(f32 x) {
  float x = x * (1.0 / (M_PI * 2.0));
  float x = x - round(x);
  return mul_add(mul_add(mul_add(mul_add(mul_add(mul_add(mul_add(mul_add((from_bits(1047347613)), x * x, -from_bits(1071090146)), x * x, from_bits(1090295415)), x * x, -from_bits(1104372952)), x * x, from_bits(1114700356)), x * x, -from_bits(1118497250)), x * x, from_bits(1115807992)), x * x, -from_bits(1100868070)), x * x, from_bits(1065353216));
}

f32 x_tan(f32 x) {
  float x = x * (1.0 / (M_PI));
  float x = x - round(x);
  float recip = 1.0 / (x * x - 0.25);
  float y = mul_add(mul_add(mul_add(mul_add(mul_add(mul_add(mul_add((-from_bits(1101493065)), x * x, from_bits(1099438785)), x * x, -from_bits(1085039819)), x * x, from_bits(1063103110)), x * x, -from_bits(1013769954)), x * x, from_bits(1041011909)), x * x, from_bits(1057932675)), x * x, -from_bits(1061752793)) * x;
  return y * recip;
}

struct Tuple{ f32 t0; f32 t1 } x_sin_cos(f32 x) {
  return (struct Tuple){t0: sin(x), t1: cos(x)};
}

