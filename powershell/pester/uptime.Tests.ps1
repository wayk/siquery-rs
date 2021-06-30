Import-Module "$PSScriptRoot/../siquery"

Describe 'uptime table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$uptime = $allSchemas | Where-Object { $_.BaseName -eq 'uptime' }
				$uptime | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$uptime = $allSchemas | Where-Object { $_.BaseName -eq 'uptime' }
				$schema = Get-Content $uptime.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$uptime = Get-Siq 'uptime' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'uptime' }).FullName -Raw

				$json = $uptime | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}