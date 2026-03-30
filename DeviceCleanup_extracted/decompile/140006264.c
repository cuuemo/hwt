/*
 * func-name: sub_140006264
 * func-address: 0x140006264
 * callers: 0x140002b64, 0x1400052dc, 0x140006850
 * callees: 0x140001898, 0x1400018d4, 0x140006128
 */

__int64 __fastcall sub_140006264(HWND a1, const CHAR *a2, const CHAR *a3, int a4)
{
  int cchWideChar; // ebx
  WCHAR *lpWideCharStr; // rsi
  int v10; // ebx
  WCHAR *v11; // rdi
  unsigned int v12; // ebx

  cchWideChar = MultiByteToWideChar(0, 0, a2, -1, 0, 0) + 1;
  lpWideCharStr = (WCHAR *)sub_140001898(saturated_mul(cchWideChar, 2u));
  MultiByteToWideChar(0, 0, a2, -1, lpWideCharStr, cchWideChar);
  v10 = MultiByteToWideChar(0, 0, a3, -1, 0, 0) + 1;
  v11 = (WCHAR *)sub_140001898(saturated_mul(v10, 2u));
  MultiByteToWideChar(0, 0, a3, -1, v11, v10);
  v12 = sub_140006128(a1, lpWideCharStr, v11, a4);
  sub_1400018D4(lpWideCharStr);
  sub_1400018D4(v11);
  return v12;
}
