/*
 * func-name: sub_1400072D8
 * func-address: 0x1400072d8
 * callers: 0x1400052dc, 0x140006850
 * callees: none
 */

_BOOL8 sub_1400072D8()
{
  BOOL IsMember; // [rsp+70h] [rbp+8h] BYREF
  _SID_IDENTIFIER_AUTHORITY pIdentifierAuthority; // [rsp+78h] [rbp+10h] BYREF
  PSID SidToCheck; // [rsp+80h] [rbp+18h] BYREF

  pIdentifierAuthority.Value[5] = 5;
  IsMember = 0;
  pIdentifierAuthority.Value[0] = 0;
  pIdentifierAuthority.Value[1] = 0;
  pIdentifierAuthority.Value[2] = 0;
  pIdentifierAuthority.Value[3] = 0;
  pIdentifierAuthority.Value[4] = 0;
  SidToCheck = 0;
  if ( AllocateAndInitializeSid(&pIdentifierAuthority, 2u, 0x20u, 0x220u, 0, 0, 0, 0, 0, 0, &SidToCheck) && SidToCheck )
  {
    CheckTokenMembership(0, SidToCheck, &IsMember);
    FreeSid(SidToCheck);
  }
  return IsMember;
}
