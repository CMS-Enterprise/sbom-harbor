use crate::entities::cyclonedx::Bom;
use crate::entities::packages::{Finding, Purl};
use crate::services::findings::FindingProvider;
use crate::services::snyk::adapters::Issue;
use crate::services::snyk::SnykService;
use crate::Error;
use platform::mongodb::{Context, Service};
use tracing::log::debug;

impl Service<Finding> for SnykService {
    fn cx(&self) -> &Context {
        &self.cx
    }
}

impl FindingProvider for SnykService {
    async fn sync(&self) -> Result<(), Error> {
        // let mut distinct = HashMap::<&str, Vec<SnykXRef>>::new();
        let purls: Vec<Purl> = self.list().await?;

        for purl in purls.clone() {
            let _raw_purl = purl.id.clone();
            // let raw_purl = raw_purl.as_str();
            // match distinct.get::<&Vec<SnykXRef>>(raw_purl.as_ref()) {
            //     None => { distinct.insert(raw_purl, vec![]);}
            //     Some(refs) => {
            //
            //     }
            // }
        }

        for purl in purls {
            // This is a BUG!!!  Could and probably DO call same purl numerous times.
            //     I need to do some work to make sure I'm calling a distinct set of Purl/Org Id
            //     combos and can map it back to the right project.
            for r in purl.snyk_refs {
                let org_id = r.org_id.clone();

                let issues = match self.issues(org_id.as_str(), purl.id.as_str()).await {
                    Ok(i) => i,
                    Err(e) => {
                        debug!("failed to get issues for purl: {}", purl.id);
                        continue;
                    }
                };

                let mut issues = match issues {
                    None => {
                        continue;
                    }
                    Some(issues) => issues,
                };

                for issue in issues.iter_mut() {
                    match self.insert(issue).await {
                        Ok(_) => {}
                        Err(e) => {
                            debug!("failed to insert issue for purl: {}", purl.id);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl SnykService {
    pub async fn sbom_issues(
        &self,
        snyk_ref: SnykXRef,
        bom: &Bom,
    ) -> Result<Option<Vec<Issue>>, Error> {
        let issues = Vec::<Issue>::new();
        let purls = bom.extract_purls(snyk_ref);

        let purls = match purls {
            None => return Ok(None),
            Some(p) => p,
        };

        for purl in purls {
            match self.issues(org_id, purl.as_str()).await? {
                None => {}
                Some(_) => {}
            }
        }

        if issues.is_empty() {
            return Ok(None);
        }

        Ok(Some(issues))
    }

    pub async fn issues(&self, org_id: &str, purl: &str) -> Result<Option<Vec<Issue>>, Error> {
        let issues = match self.client.get_issues(org_id, purl).await {
            Ok(issues) => issues,
            Err(e) => {
                return Err(Error::Snyk(format!(
                    "snyk::issues: purl - {} - {}",
                    purl, e
                )));
            }
        };

        let issues = match issues {
            None => {
                return Ok(None);
            }
            Some(issues) => issues,
        };

        if issues.is_empty() {
            return Ok(None);
        }

        let mut results = vec![];
        issues
            .iter()
            .for_each(|inner| results.push(Issue::new(purl.to_string(), inner.clone())));

        Ok(Some(results))
    }
}
