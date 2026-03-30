/*
 * func-name: sub_140007B74
 * func-address: 0x140007b74
 * callers: 0x140003cec
 * callees: 0x140001dc0, 0x140006128
 */

__int64 __fastcall sub_140007B74(LPCWSTR lpParameters)
{
  HINSTANCE v1; // rbx
  WCHAR *v2; // rax
  WCHAR String1[256]; // [rsp+30h] [rbp-418h] BYREF
  WCHAR szVolumePathName[268]; // [rsp+230h] [rbp-218h] BYREF

  v1 = ShellExecuteW(0, L"runas", &Parameters, lpParameters, 0, 1);
  if ( (unsigned __int64)v1 > 0x20 )
    return 1;
  v2 = sub_140001DC0();
  wsprintfW(String1, L"ShellExecute failed. Error=%s hi=0x%p", v2, v1);
  if ( GetLastError() == 3
    && GetVolumePathNameW(&pszPath, szVolumePathName, 0x104u)
    && GetDriveTypeW(szVolumePathName) == 4 )
  {
    lstrcatW(String1, L" \r\n(probably because the app is located on a network drive)");
  }
  sub_140006128(0, String1, L"DeviceCleanup", 16);
  return 0;
}
