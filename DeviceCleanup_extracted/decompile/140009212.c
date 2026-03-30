/*
 * func-name: WinVerifyTrust
 * func-address: 0x140009212
 * callers: 0x140008ea0
 * callees: none
 */

// attributes: thunk
LONG __stdcall WinVerifyTrust(HWND hwnd, GUID *pgActionID, LPVOID pWVTData)
{
  return __imp_WinVerifyTrust(hwnd, pgActionID, pWVTData);
}
