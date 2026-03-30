/*
 * func-name: sub_140003584
 * func-address: 0x140003584
 * callers: 0x1400036d4, 0x14000453c
 * callees: 0x140003544, 0x140007980
 */

int __fastcall sub_140003584(int a1, int a2, __int64 a3)
{
  int result; // eax
  LPARAM lParam; // [rsp+20h] [rbp-68h] BYREF
  BOOL v8; // [rsp+28h] [rbp-60h]
  WCHAR *v9; // [rsp+38h] [rbp-50h]
  int v10; // [rsp+40h] [rbp-48h]

  if ( a3 == 1 )
  {
    result = sub_140003544(a1, a2);
  }
  else
  {
    v8 = a3;
    v10 = 256;
    v9 = &String1;
    SendMessageW(hWnd, 0x1073u, a1, (LPARAM)&lParam);
    v8 = a3;
    v10 = 256;
    v9 = &word_140010A40;
    SendMessageW(hWnd, 0x1073u, a2, (LPARAM)&lParam);
    if ( a3 == dword_14000F46C )
      result = sub_140007980(&String1, &word_140010A40);
    else
      result = lstrcmpW(&String1, &word_140010A40);
  }
  if ( !result )
  {
    v10 = 256;
    v8 = a3 == 0;
    v9 = &String1;
    SendMessageW(hWnd, 0x1073u, a1, (LPARAM)&lParam);
    v8 = a3 == 0;
    v10 = 256;
    v9 = &word_140010A40;
    SendMessageW(hWnd, 0x1073u, a2, (LPARAM)&lParam);
    result = lstrcmpW(&String1, &word_140010A40);
  }
  if ( !dword_14000F478[a3] )
    return -result;
  return result;
}
