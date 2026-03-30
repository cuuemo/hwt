/*
 * func-name: sub_140007268
 * func-address: 0x140007268
 * callers: 0x1400036d4, 0x140007980
 * callees: none
 */

const WCHAR *__fastcall sub_140007268(PCWSTR pszFirst, PCWSTR pszSrch)
{
  unsigned __int64 v4; // rsi
  int v5; // eax
  __int64 v6; // rbx
  const WCHAR *v7; // rax

  v4 = lstrlenW(pszFirst);
  v5 = lstrlenW(pszSrch);
  v6 = 0;
  if ( !v4 || !v5 || v5 > v4 )
    return 0;
  v7 = StrStrIW(pszFirst, pszSrch);
  if ( v7 == pszFirst )
    return v7;
  return (const WCHAR *)v6;
}
