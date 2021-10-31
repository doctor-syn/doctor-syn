#include<math.h>

typedef float f32;
typedef int i32;
typedef unsigned u32;

inline f32 f32_mul_add(f32 a, f32 b, f32 c) {
    return a * b + c;
}

inline f32 f32_select(int a, f32 b, f32 c) {
    return a ? b : c;
}

inline f32 f32_from_bits(u32 x) {
    union {
        float f;
        unsigned x;
    } u;
    u.x = x;
    return u.f;
}

inline u32 f32_to_bits(f32 f) {
    union {
        float f;
        unsigned x;
    } u;
    u.f = f;
    return u.x;
}

const f32 PI = (f32)M_PI;
const f32 LOG2_E = (f32)M_LOG2E;
const f32 LOG2_10 = (f32)M_LN10 / M_LN2;

f32 f32_sin(f32 arg) {
  f32 scaled = arg * (1.0 / (PI * 2.0));
  f32 x = scaled - f32_round(scaled);
  return f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((-f32_from_bits(1058770289)), x * x, f32_from_bits(1081164794)), x * x, -f32_from_bits(1097945697)), x * x, f32_from_bits(1109932662)), x * x, -f32_from_bits(1117350231)), x * x, f32_from_bits(1117992419)), x * x, -f32_from_bits(1109745127)), x * x, f32_from_bits(1086918619)) * x;
}

f32 f32_cos(f32 arg) {
  f32 scaled = arg * (1.0 / (PI * 2.0));
  f32 x = scaled - f32_round(scaled);
  return f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((f32_from_bits(1047347613)), x * x, -f32_from_bits(1071090146)), x * x, f32_from_bits(1090295415)), x * x, -f32_from_bits(1104372952)), x * x, f32_from_bits(1114700356)), x * x, -f32_from_bits(1118497250)), x * x, f32_from_bits(1115807992)), x * x, -f32_from_bits(1100868070)), x * x, f32_from_bits(1065353216));
}

f32 f32_tan(f32 arg) {
  f32 scaled = arg * (1.0 / PI);
  f32 x = scaled - f32_round(scaled);
  f32 recip = 1.0 / (x * x - 0.25);
  f32 y = f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((-f32_from_bits(1101493065)), x * x, f32_from_bits(1099438785)), x * x, -f32_from_bits(1085039819)), x * x, f32_from_bits(1063103110)), x * x, -f32_from_bits(1013769954)), x * x, f32_from_bits(1041011909)), x * x, f32_from_bits(1057932675)), x * x, -f32_from_bits(1061752793)) * x;
  return y * recip;
}

struct Tuple{ f32 t0; f32 t1 } f32_sin_cos(f32 arg) {
  return (struct Tuple){t0: f32_sin(arg), t1: f32_cos(arg)};
}

f32 f32_asin(f32 arg) {
  const f32 LIM = 0.9;;
  f32 c = f32_select(arg < 0.0, -PI / 2.0, PI / 2.0);
  f32 s = f32_select(arg < 0.0, -1.0, 1.0);
  f32 x0 = arg;
  f32 x = f32_select(arg * arg < LIM * LIM, x, f32_sqrt((1.0 - x * x)));
  f32 y = f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((f32_from_bits(1220607974)), x * x, -f32_from_bits(1238634524)), x * x, f32_from_bits(1246143248)), x * x, -f32_from_bits(1246649024)), x * x, f32_from_bits(1241607372)), x * x, -f32_from_bits(1229443293)), x * x, f32_from_bits(1211705078)), x * x, -f32_from_bits(1186972402)), x * x, f32_from_bits(1153576661)), x * x, -f32_from_bits(1108437697)), x * x, f32_from_bits(1066489688)) * x;
  return f32_select(x0 * x0 < LIM * LIM, y, c - y * s);
}

f32 f32_acos(f32 arg) {
  const f32 LIM = 0.9;;
  f32 c = f32_select(arg < 0.0, PI, 0.0);
  f32 s = f32_select(arg < 0.0, 1.0, -1.0);
  f32 x = f32_select(arg * arg < LIM * LIM, x, f32_sqrt((1.0 - x * x)));
  f32 y = f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((f32_from_bits(1220607974)), x * x, -f32_from_bits(1238634524)), x * x, f32_from_bits(1246143248)), x * x, -f32_from_bits(1246649024)), x * x, f32_from_bits(1241607372)), x * x, -f32_from_bits(1229443293)), x * x, f32_from_bits(1211705078)), x * x, -f32_from_bits(1186972402)), x * x, f32_from_bits(1153576661)), x * x, -f32_from_bits(1108437697)), x * x, f32_from_bits(1066489688)) * x;
  return f32_select(arg * arg < LIM * LIM, PI / 2.0 - y, c - y * s);
}

f32 f32_atan(f32 arg) {
  const f32 LIM = 1.0;;
  f32 c = f32_select(arg < 0.0, -PI / 2.0, PI / 2.0);
  f32 small = f32_abs(arg) < LIM;
  f32 x = f32_select(small, arg, f32_recip(arg));
  f32 y = f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((-f32_from_bits(1162019910)), x * x, f32_from_bits(1183891392)), x * x, -f32_from_bits(1194961313)), x * x, f32_from_bits(1199976839)), x * x, -f32_from_bits(1198438416)), x * x, f32_from_bits(1191544479)), x * x, -f32_from_bits(1178025219)), x * x, f32_from_bits(1158538705)), x * x, -f32_from_bits(1129118537)), x * x, f32_from_bits(1086532386)), x * x, f32_from_bits(1064851362)) * x;
  return f32_select(small, y, c - y);
}

f32 f32_atan2(f32 y, f32 x) {
  f32 offset180 = f32_select(y < 0.0, -PI, PI);
  f32 x1 = f32_select(x < 0.0, -x, x);
  f32 y1 = f32_select(x < 0.0, -y, y);
  f32 offset1 = f32_select(x < 0.0, offset180, 0.0);
  f32 offset90 = f32_select(y < 0.0, -PI / 2.0, PI / 2.0);
  f32 x2 = f32_select(f32_abs(y1) > x1, y1, x1);
  f32 y2 = f32_select(f32_abs(y1) > x1, -x1, y1);
  f32 offset2 = f32_select(f32_abs(y1) > x1, offset1 + offset90, offset1);
  f32 x3 = y2 / x2;
  f32 y3 = f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((-f32_from_bits(1119840012)), x3 * x3, f32_from_bits(1137999842)), x3 * x3, -f32_from_bits(1145036515)), x3 * x3, f32_from_bits(1144168768)), x3 * x3, -f32_from_bits(1135684044)), x3 * x3, f32_from_bits(1118301297)), x3 * x3, -f32_from_bits(1086827175)), x3 * x3, f32_from_bits(1065814464)) * x3;
  return y3 + offset2;
}

f32 f32_exp(f32 arg) {
  return f32_exp2(arg * LOG2_E);
}

f32 f32_exp2(f32 arg) {
  f32 r = f32_round(arg);
  f32 mul = f32_from_bits((u32)(f32_mul_add(r, 8388608, 1065353216)));
  f32 x = x - r;
  return f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((f32_from_bits(958543394)), x, f32_from_bits(984597629)), x, f32_from_bits(1008571634)), x, f32_from_bits(1029920534)), x, f32_from_bits(1047920112)), x, f32_from_bits(1060205081)), x, f32_from_bits(1065353216)) * mul;
}

f32 f32_exp_m1(f32 arg) {
  f32 scaled = arg * LOG2_E;
  f32 r = f32_round(scaled);
  f32 mul = f32_from_bits((u32)(f32_mul_add(r, 8388608, 1065353216)));
  f32 x = scaled - r;
  return f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((f32_from_bits(958543394)), x, f32_from_bits(984597629)), x, f32_from_bits(1008571634)), x, f32_from_bits(1029920534)), x, f32_from_bits(1047920112)), x, f32_from_bits(1060205081)), x, f32_from_bits(0)) * mul + (mul - 1.0);
}

f32 f32_ln(f32 arg) {
  return f32_log2(arg) * (1.0 / LOG2_E);
}

f32 f32_ln_1p(f32 arg) {
  f32 exponent = (f32)(f32_to_bits((arg + 1.0)) >> 23) - 127;
  f32 x = f32_select(exponent == 0, x, f32_from_bits((f32_to_bits((x + 1.0)) & 8388607) | 1065353216) - 1.0);
  f32 y = f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((-f32_from_bits(1007773536)), x, f32_from_bits(1028608721)), x, -f32_from_bits(1040907604)), x, f32_from_bits(1047965318)), x, -f32_from_bits(1051835875)), x, f32_from_bits(1056251114)), x, -f32_from_bits(1060675479)), x, f32_from_bits(1069066780)), x, f32_from_bits(0));
  return (y + ((f32)exponent)) * (1.0 / LOG2_E);
}

f32 f32_log2(f32 arg) {
  i32 exponent = (i32)(f32_to_bits(arg) >> 23) - 127;
  f32 x = f32_from_bits((f32_to_bits(arg) & (8388608 - 1)) | 1065353216) - 1.5;
  f32 y = f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add(f32_mul_add((-f32_from_bits(1007773536)), x, f32_from_bits(1014476229)), x, -f32_from_bits(1017745593)), x, f32_from_bits(1025038344)), x, -f32_from_bits(1032976966)), x, f32_from_bits(1041364798)), x, -f32_from_bits(1050944880)), x, f32_from_bits(1064712249)), x, f32_from_bits(1058390042));
  return y + ((f32)exponent);
}

f32 f32_log10(f32 arg) {
  return f32_log2(arg) * (1.0 / LOG2_10);
}

f32 f32_log(f32 arg, f32 base) {
  return f32_log2(arg) / f32_log2(base);
}

f32 f32_powi(f32 x, i32 y) {
  f32 a = x;
  f32 p = f32_abs(y);
  f32 b = f32_select((p & (1 << 0)) != 0, a, 1.0);
  f32 a1 = a1 * a1;
  f32 b1 = f32_select((p & (1 << 1)) != 0, b * a, b);
  f32 a2 = a2 * a2;
  f32 b2 = f32_select((p & (1 << 2)) != 0, b1 * a, b);
  f32 a3 = a3 * a3;
  f32 b3 = f32_select((p & (1 << 3)) != 0, b2 * a, b);
  b4 = f32_select(p < 16, b3, f32_powf(x, (f32)p));
  return f32_select(y < 0, f32_recip(b4), b4);
}

f32 f32_powf(f32 arg, f32 y) {
  return f32_exp2(f32_log2(arg) * y);
}

f32 f32_sinh(f32 x) {
  a = f32_mul_add(x, std :: f32 :: consts :: LOG2_E, -1.0);
  b = f32_mul_add(x, -std :: f32 :: consts :: LOG2_E, -1.0);
  return f32_exp2(a) - f32_exp2(b);
}

f32 f32_cosh(f32 x) {
  a = f32_mul_add(x, std :: f32 :: consts :: LOG2_E, -1.0);
  b = f32_mul_add(x, -std :: f32 :: consts :: LOG2_E, -1.0);
  return f32_exp2(a) + f32_exp2(b);
}

f32 f32_tanh(f32 x) {
  exp2x = f32_exp2(x * (std :: f32 :: consts :: LOG2_E * 2.0));
  return (exp2x - 1.0) / (exp2x + 1.0);
}

f32 f32_asinh(f32 x) {
  return f32_ln(x + f32_sqrt((x * x + 1.0)));
}

f32 f32_acosh(f32 x) {
  return f32_ln(x + f32_sqrt((x * x - 1.0)));
}

f32 f32_atanh(f32 x) {
  return (f32_ln(1.0 + x) - f32_ln(1.0 - x)) * 0.5;
}

f32 f32_sqrt(f32 x) {
  f32 r = f32_sqrt_approx(x);
  f32 y = r + (x - r * r) / (2.0 * r);
  return y;
}

f32 f32_cbrt(f32 x) {
  f32 r = f32_cbrt_approx(f32_abs(x));
  f32 y = r + (f32_abs(x) - r * r * r) / (3.0 * r * r);
  return f32_copysign(y, x);
}

f32 f32_hypot(f32 x, f32 y) {
  f32 xgty = f32_abs(x) > f32_abs(y);
  f32 x2 = f32_select(xgty, x, y);
  f32 y2 = f32_select(xgty, y, x);
  return f32_select(f32_abs(x2) <= f32 :: MIN_POSITIVE, x2, f32_abs(x2) * f32_sqrt((1.0 + (y2 / x2) * (y2 / x2))));
}

f32 f32_recip(f32 x) {
  f32 r = f32_recip_approx(x);
  f32 r1 = r * (2.0 - x * r);
  f32 r2 = r1 * (2.0 - x * r1);
  f32 r3 = r2 * (2.0 - x * r2);
  return r3;
}

f32 f32_negate_on_odd(f32 x, f32 y) {
  sign_bit = (u32)((((i32)x) & 1) << 31);
  return f32_from_bits(sign_bit ^ f32_to_bits(y));
}

f32 f32_recip_approx(f32 x) {
  y = f32_from_bits((u32)((f32_mul_add(((f32)f32_to_bits(f32_abs(x))), -1.0, (f32)1065353216 * 2.0))));
  return f32_copysign((y - 0.08), x);
}

f32 f32_sqrt_approx(f32 x) {
  y = f32_from_bits((u32)((f32_mul_add(((f32)f32_to_bits(f32_abs(x))), 0.5, (f32)1065353216 * 0.5))));
  return y - 0.08;
}

f32 f32_cbrt_approx(f32 x) {
  y = f32_from_bits((u32)((f32_mul_add(((f32)f32_to_bits(f32_abs(x))), 1.0 / 3.0, (f32)1065353216 * 2.0 / 3.0))));
  return f32_copysign((y - 0.08), x);
}

