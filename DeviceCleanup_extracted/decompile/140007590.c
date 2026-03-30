/*
 * func-name: sub_140007590
 * func-address: 0x140007590
 * callers: 0x1400052dc
 * callees: none
 */

int __fastcall sub_140007590(WCHAR *lpString2, WCHAR *pszPath)
{
  LPWSTR FileNameW; // rax

  if ( pszPath != lpString2 )
    lstrcpyW(pszPath, lpString2);
  FileNameW = PathFindFileNameW(pszPath);
  if ( FileNameW > pszPath && *(FileNameW - 1) == 92 )
    *(FileNameW - 1) = 0;
  return lstrlenW(pszPath);
}
