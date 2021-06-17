function Get-SiqTableName {
	[CmdletBinding()]
	param()

	$siq_exe = Find-SiqExecutable
	$tables = (& $siq_exe '-l') -Split '`n'
	return $tables
}
