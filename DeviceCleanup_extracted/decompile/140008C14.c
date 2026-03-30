/*
 * func-name: sub_140008C14
 * func-address: 0x140008c14
 * callers: 0x140008fc8
 * callees: 0x140001854
 */

__int64 __fastcall sub_140008C14(__int64 a1, __int64 a2, __int64 a3, __int16 a4)
{
  unsigned int v4; // edi
  HMODULE ModuleHandleA; // rax
  BOOL (__stdcall *CreateProcessWithTokenW)(HANDLE, DWORD, LPCWSTR, LPWSTR, DWORD, LPVOID, LPCWSTR, LPSTARTUPINFOW, LPPROCESS_INFORMATION); // rax
  HANDLE CurrentProcess; // rax
  DWORD LastError; // ebx
  HWND ShellWindow; // rax
  HANDLE v15; // rax
  void *v16; // rbx
  DWORD dwProcessId; // [rsp+50h] [rbp-C8h] BYREF
  HANDLE hExistingToken; // [rsp+58h] [rbp-C0h] BYREF
  HANDLE phNewToken; // [rsp+60h] [rbp-B8h] BYREF
  HANDLE TokenHandle; // [rsp+68h] [rbp-B0h] BYREF
  struct _TOKEN_PRIVILEGES Luid; // [rsp+70h] [rbp-A8h] BYREF
  __int64 v22; // [rsp+80h] [rbp-98h]
  int v23; // [rsp+90h] [rbp-88h] BYREF
  _BYTE v24[8]; // [rsp+98h] [rbp-80h] BYREF
  const wchar_t *v25; // [rsp+A0h] [rbp-78h]
  int v26; // [rsp+CCh] [rbp-4Ch]
  __int16 v27; // [rsp+D0h] [rbp-48h]

  v4 = 0;
  if ( !qword_140017060 )
  {
    ModuleHandleA = GetModuleHandleA("AdvApi32");
    if ( ModuleHandleA )
    {
      CreateProcessWithTokenW = (BOOL (__stdcall *)(HANDLE, DWORD, LPCWSTR, LPWSTR, DWORD, LPVOID, LPCWSTR, LPSTARTUPINFOW, LPPROCESS_INFORMATION))GetProcAddress(ModuleHandleA, "CreateProcessWithTokenW");
      qword_140017060 = (__int64 (__fastcall *)(_QWORD, _QWORD, _QWORD, _QWORD, _DWORD, _QWORD, _QWORD, _QWORD, _QWORD))CreateProcessWithTokenW;
    }
    else
    {
      CreateProcessWithTokenW = (BOOL (__stdcall *)(HANDLE, DWORD, LPCWSTR, LPWSTR, DWORD, LPVOID, LPCWSTR, LPSTARTUPINFOW, LPPROCESS_INFORMATION))qword_140017060;
    }
    if ( !CreateProcessWithTokenW )
      return 0;
  }
  hExistingToken = 0;
  phNewToken = 0;
  dwProcessId = 0;
  TokenHandle = 0;
  CurrentProcess = GetCurrentProcess();
  if ( !OpenProcessToken(CurrentProcess, 0x20u, &TokenHandle) )
    goto LABEL_8;
  Luid.PrivilegeCount = 1;
  LookupPrivilegeValueW(0, L"SeIncreaseQuotaPrivilege", &Luid.Privileges[0].Luid);
  Luid.Privileges[0].Attributes = 2;
  AdjustTokenPrivileges(TokenHandle, 0, &Luid, 0, 0, 0);
  LastError = GetLastError();
  CloseHandle(TokenHandle);
  if ( LastError )
    return 0;
  ShellWindow = GetShellWindow();
  if ( !ShellWindow )
    return 0;
  GetWindowThreadProcessId(ShellWindow, &dwProcessId);
  if ( !dwProcessId )
    return 0;
  v15 = OpenProcess(0x400u, 0, dwProcessId);
  v16 = v15;
  if ( !v15 )
  {
LABEL_8:
    GetLastError();
    return 0;
  }
  if ( !OpenProcessToken(v15, 2u, &hExistingToken) )
    goto LABEL_16;
  if ( !DuplicateTokenEx(hExistingToken, 0x18Bu, 0, SecurityImpersonation, TokenPrimary, &phNewToken) )
    goto LABEL_16;
  *(_QWORD *)&Luid.PrivilegeCount = 0;
  v23 = 0;
  *(_QWORD *)&Luid.Privileges[0].Luid.HighPart = 0;
  v22 = 0;
  sub_140001854(v24, 0, 0x60u);
  v25 = L"default";
  v26 = 1;
  v27 = a4;
  if ( (unsigned int)qword_140017060(phNewToken, 0, a1, a2, 0, 0, a3, &v23, &Luid) )
  {
    CloseHandle(*(HANDLE *)&Luid.PrivilegeCount);
    *(_QWORD *)&Luid.PrivilegeCount = 0;
    CloseHandle(*(HANDLE *)&Luid.Privileges[0].Luid.HighPart);
    v4 = 1;
  }
  else
  {
LABEL_16:
    GetLastError();
  }
  CloseHandle(hExistingToken);
  CloseHandle(phNewToken);
  CloseHandle(v16);
  return v4;
}
