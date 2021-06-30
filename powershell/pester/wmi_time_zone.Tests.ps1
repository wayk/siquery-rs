Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_time_zone table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_time_zone = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_time_zone' }
				$wmi_time_zone | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_time_zone = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_time_zone' }
				$schema = Get-Content $wmi_time_zone.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_time_zone = Get-Siq 'wmi_time_zone' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_time_zone' }).FullName -Raw

				$json = $wmi_time_zone | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}