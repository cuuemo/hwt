/*
 * func-name: sub_140007DB0
 * func-address: 0x140007db0
 * callers: 0x140002008
 * callees: none
 */

BOOL __fastcall sub_140007DB0(LPCSTR lpAppName, LPCSTR lpKeyName, int a3, const CHAR *a4)
{
  CHAR String[40]; // [rsp+20h] [rbp-28h] BYREF

  wsprintfA(String, "%i", a3);
  return WritePrivateProfileStringA(lpAppName, lpKeyName, String, a4);
}
