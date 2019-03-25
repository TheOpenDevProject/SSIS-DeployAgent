## Maybe the worlds first CodeDeploy agent for SSIS.

Deploying SSIS packages is an ancient ritual and it sucks, there is basically zero automation for the process. Microsoft(TM) SSIS is a super powerful ETL tool that can handle **billions** of records and is used by the largest organisations in the world, this has never made its deployment model suck less :anguished:.

---

## It's time to bring SSIS into the 21st century.

**Project Goals:**

[] - Highly configurable: Any branching model, any deployment structure at any scale.

[] - Compatible with any CI/CD platform that can make web requests. (Pipeline mode)

[] - Provide webhooks for git platforms that support such to trigger builds / deployments. (Stand alone mode)

[] - Test Automation: Create an isolated environment to run SSIS package in and report the execution results (Via SSIS Logging)

[] - Deployment notifications

---

### We need your help

Over the coming days I will open more and more issues for people to pickup and work on. We're working mostly in rust and SQL so a solid grasp of those would be a very big help.
