/*
 * func-name: sub_140008AEC
 * func-address: 0x140008aec
 * callers: 0x140003010
 * callees: 0x140001854, 0x140007530
 */

HWND __fastcall sub_140008AEC(int a1, HWND a2, __int64 a3)
{
  LPARAM v5; // rdi
  HWND DlgItem; // r12
  HWND Window; // rax
  HWND v8; // rbp
  LPARAM lParam[3]; // [rsp+60h] [rbp-58h] BYREF
  struct tagRECT Rect; // [rsp+78h] [rbp-40h] BYREF
  __int64 v12; // [rsp+90h] [rbp-28h]

  v5 = a1;
  if ( !a1 )
    return 0;
  if ( !a2 )
    return 0;
  if ( !a3 )
    return 0;
  DlgItem = GetDlgItem(a2, a1);
  Window = CreateWindowExW(
             0,
             L"tooltips_class32",
             0,
             0x80000001,
             0x80000000,
             0x80000000,
             0x80000000,
             0x80000000,
             a2,
             0,
             0,
             0);
  v8 = Window;
  if ( !DlgItem || !Window )
    return 0;
  sub_140001854((char *)lParam + 4, 0, 0x44u);
  lParam[0] = 0x1000000044LL;
  lParam[1] = (LPARAM)a2;
  lParam[2] = v5;
  v12 = a3;
  GetWindowRect(DlgItem, &Rect);
  sub_140007530(a2, (struct tagPOINT *)&Rect);
  SendMessageW(v8, 0x432u, 0, (LPARAM)lParam);
  return v8;
}
