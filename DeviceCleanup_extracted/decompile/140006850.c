/*
 * func-name: sub_140006850
 * func-address: 0x140006850
 * callers: 0x140003cec
 * callees: 0x140001854, 0x1400018e8, 0x140006264, 0x140006378, 0x1400063e8, 0x14000641c, 0x140007000, 0x1400070b8, 0x140007184, 0x1400072d8, 0x1400073d8, 0x1400075d4
 */

__int64 sub_140006850()
{
  HWND DesktopWindow; // rax
  const CHAR *v1; // rdx
  HMODULE v3; // rax
  HMODULE v4; // rdi
  BOOL (__stdcall *SRSetRestorePointW)(PRESTOREPOINTINFOW, PSTATEMGRSTATUS); // rbp
  int v6; // esi
  BOOL v7; // ebx
  unsigned int v8; // r12d
  const char *v9; // rax
  unsigned int v10[4]; // [rsp+20h] [rbp-438h] BYREF
  int v11; // [rsp+30h] [rbp-428h] BYREF
  int v12[3]; // [rsp+34h] [rbp-424h] BYREF
  WCHAR String1[256]; // [rsp+40h] [rbp-418h] BYREF
  CHAR v14[512]; // [rsp+240h] [rbp-218h] BYREF
  unsigned int v15; // [rsp+460h] [rbp+8h] BYREF

  if ( !(unsigned int)sub_1400075D4() )
  {
    DesktopWindow = GetDesktopWindow();
    v1 = "This Windows has no System Restore";
LABEL_3:
    MessageBoxA(DesktopWindow, v1, "Error", 0x10u);
    return 0;
  }
  sub_140006378();
  sub_14000641C();
  v3 = (HMODULE)sub_1400073D8(L"SrClient.dll");
  v4 = v3;
  if ( !v3
    || (SRSetRestorePointW = (BOOL (__stdcall *)(PRESTOREPOINTINFOW, PSTATEMGRSTATUS))GetProcAddress(
                                                                                        v3,
                                                                                        "SRSetRestorePointW")) == 0 )
  {
    DesktopWindow = GetDesktopWindow();
    v1 = "System Restore is not available";
    goto LABEL_3;
  }
  v15 = -1;
  v6 = 0;
  v7 = 0;
  if ( dword_140010680 >= 1538 )
  {
    if ( (unsigned int)sub_140007000(
                         -2147483646,
                         L"Software\\Microsoft\\Windows NT\\CurrentVersion\\SystemRestore",
                         L"SystemRestorePointCreationFrequency",
                         &v15) )
      v6 = 1;
    else
      v7 = GetLastError() == 2;
    if ( v15
      && !(unsigned int)sub_1400070B8(
                          -2147483646,
                          L"Software\\Microsoft\\Windows NT\\CurrentVersion\\SystemRestore",
                          L"SystemRestorePointCreationFrequency",
                          0) )
    {
      v7 = 0;
      v6 = 0;
    }
  }
  v8 = 0;
  sub_140001854(v12, 0, 0x20Cu);
  v10[0] = 0;
  v10[1] = 0;
  v10[2] = 0;
  v12[0] = 16;
  lstrcpyW(String1, L"DeviceCleanup");
  v11 = 100;
  if ( ((unsigned int (__fastcall *)(int *, unsigned int *))SRSetRestorePointW)(&v11, v10)
    && (v11 = 101, ((unsigned int (__fastcall *)(int *, unsigned int *))SRSetRestorePointW)(&v11, v10)) )
  {
    MessageBoxA(0, "System Restore Point successfully created", "DeviceCleanup", 0x40u);
    v8 = 1;
  }
  else
  {
    v9 = sub_1400018E8(v10[0]);
    wsprintfA(v14, "Error %s creating system restore point", v9);
    if ( !(unsigned int)sub_1400072D8() )
      lstrcatA(v14, "\r\nAdmin privileges are required");
    sub_140006264(0, v14, "DeviceCleanup", 16);
  }
  if ( v6 )
  {
    sub_1400070B8(
      -2147483646,
      L"Software\\Microsoft\\Windows NT\\CurrentVersion\\SystemRestore",
      L"SystemRestorePointCreationFrequency",
      v15);
  }
  else if ( v7 )
  {
    sub_140007184(
      HKEY_LOCAL_MACHINE,
      L"Software\\Microsoft\\Windows NT\\CurrentVersion\\SystemRestore",
      L"SystemRestorePointCreationFrequency");
  }
  FreeLibrary(v4);
  sub_1400063E8();
  return v8;
}
