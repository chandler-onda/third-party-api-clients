use anyhow::Result;

use crate::Client;

pub struct Issues {
    client: Client,
}

impl Issues {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Issues { client }
    }

    /**
     * List issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/issues` endpoint.
     *
     * List issues assigned to the authenticated user across all visible repositories including owned repositories, member
     * repositories, and organization repositories. You can use the `filter` query parameter to fetch issues that are not
     * necessarily assigned to you.
     *
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. Can be one of:  
     *  \* `assigned`: Issues assigned to you  
     *  \* `created`: Issues created by you  
     *  \* `mentioned`: Issues mentioning you  
     *  \* `subscribed`: Issues you're subscribed to updates for  
     *  \* `all`: All issues the authenticated user can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Direction` -- One of `asc` (ascending) or `desc` (descending).
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `collab: bool`
     * * `orgs: bool`
     * * `owned: bool`
     * * `pulls: bool`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list(
        &self,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Direction,
        since: chrono::DateTime<chrono::Utc>,
        collab: bool,
        orgs: bool,
        owned: bool,
        pulls: bool,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Issue>> {
        let url = format!(
            "/issues?collab={}&direction={}&filter={}&labels={}&orgs={}&owned={}&page={}&\
             per_page={}&pulls={}&since={}&sort={}&state={}",
            format!("{}", collab),
            direction,
            filter,
            labels.to_string(),
            format!("{}", orgs),
            format!("{}", owned),
            format!("{}", page),
            format!("{}", per_page),
            format!("{}", pulls),
            since,
            sort,
            state,
        );

        self.client.get(&url).await
    }

    /**
     * List organization issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/orgs/{org}/issues` endpoint.
     *
     * List issues in an organization assigned to the authenticated user.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-organization-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. Can be one of:  
     *  \* `assigned`: Issues assigned to you  
     *  \* `created`: Issues created by you  
     *  \* `mentioned`: Issues mentioning you  
     *  \* `subscribed`: Issues you're subscribed to updates for  
     *  \* `all`: All issues the authenticated user can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Direction` -- One of `asc` (ascending) or `desc` (descending).
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_org(
        &self,
        org: &str,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Direction,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Issue>> {
        let url = format!(
            "/orgs/{}/issues?direction={}&filter={}&labels={}&page={}&per_page={}&since={}&\
             sort={}&state={}",
            crate::progenitor_support::encode_path(&org.to_string()),
            direction,
            filter,
            labels.to_string(),
            format!("{}", page),
            format!("{}", per_page),
            since,
            sort,
            state,
        );

        self.client.get(&url).await
    }

    /**
     * List assignees.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/assignees` endpoint.
     *
     * Lists the [available assignees](https://help.github.com/articles/assigning-issues-and-pull-requests-to-other-github-users/) for issues in a repository.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-assignees>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_assignees(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::User>> {
        let url = format!(
            "/repos/{}/{}/assignees?page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }

    /**
     * Check if a user can be assigned.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/assignees/{assignee}` endpoint.
     *
     * Checks if a user has permission to be assigned to an issue in this repository.
     *
     * If the `assignee` can be assigned to issues in the repository, a `204` header with no content is returned.
     *
     * Otherwise a `404` status code is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#check-if-a-user-can-be-assigned>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `assignee: &str`
     */
    pub async fn check_user_can_be_assigned(
        &self,
        owner: &str,
        repo: &str,
        assignee: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/assignees/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&assignee.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * List repository issues.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues` endpoint.
     *
     * List issues in a repository.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-repository-issues>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone: &str` -- If an `integer` is passed, it should refer to a milestone by its `number` field. If the string `*` is passed, issues with any milestone are accepted. If the string `none` is passed, issues without milestones are returned.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `assignee: &str` -- Can be the name of a user. Pass in `none` for issues with no assigned user, and `*` for issues assigned to any user.
     * * `creator: &str` -- The user that created the issue.
     * * `mentioned: &str` -- A user that's mentioned in the issue.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Direction` -- One of `asc` (ascending) or `desc` (descending).
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_repo(
        &self,
        owner: &str,
        repo: &str,
        milestone: &str,
        state: crate::types::IssuesListState,
        assignee: &str,
        creator: &str,
        mentioned: &str,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Direction,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::IssueSimple>> {
        let url = format!(
            "/repos/{}/{}/issues?assignee={}&creator={}&direction={}&labels={}&mentioned={}&\
             milestone={}&page={}&per_page={}&since={}&sort={}&state={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            assignee.to_string(),
            creator.to_string(),
            direction,
            labels.to_string(),
            mentioned.to_string(),
            milestone.to_string(),
            format!("{}", page),
            format!("{}", per_page),
            since,
            sort,
            state,
        );

        self.client.get(&url).await
    }

    /**
     * Create an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues` endpoint.
     *
     * Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://help.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in abuse rate limiting. See "[Abuse rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#abuse-rate-limits)" and "[Dealing with abuse rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-abuse-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::IssuesCreateRequest,
    ) -> Result<crate::types::Issue> {
        let url = format!(
            "/repos/{}/{}/issues",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List issue comments for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments` endpoint.
     *
     * By default, Issue Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-comments-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sort: crate::types::Sort` -- One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
     * * `direction: crate::types::Direction` -- Either `asc` or `desc`. Ignored without the `sort` parameter.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::Sort,
        direction: crate::types::Direction,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::IssueComment>> {
        let url = format!(
            "/repos/{}/{}/issues/comments?direction={}&page={}&per_page={}&since={}&sort={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            direction,
            format!("{}", page),
            format!("{}", per_page),
            since,
            sort,
        );

        self.client.get(&url).await
    }

    /**
     * Get an issue comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn get_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<crate::types::IssueComment> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Delete an issue comment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#delete-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn delete_comment(&self, owner: &str, repo: &str, comment_id: i64) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update an issue comment.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#update-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn update_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::IssuesUpdateCommentRequest,
    ) -> Result<crate::types::IssueComment> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List issue events for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-events-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_events_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::IssueEvent>> {
        let url = format!(
            "/repos/{}/{}/issues/events?page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }

    /**
     * Get an issue event.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events/{event_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-an-issue-event>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `event_id: i64`
     */
    pub async fn get_event(
        &self,
        owner: &str,
        repo: &str,
        event_id: i64,
    ) -> Result<crate::types::IssueEvent> {
        let url = format!(
            "/repos/{}/{}/issues/events/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&event_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}` endpoint.
     *
     * The API returns a [`301 Moved Permanently` status](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-redirects-redirects) if the issue was
     * [transferred](https://help.github.com/articles/transferring-an-issue-to-another-repository/) to another repository. If
     * the issue was transferred to or deleted from a repository where the authenticated user lacks read access, the API
     * returns a `404 Not Found` status. If the issue was deleted from a repository where the authenticated user has read
     * access, the API returns a `410 Gone` status. To receive webhook events for transferred and deleted issues, subscribe
     * to the [`issues`](https://docs.github.com/webhooks/event-payloads/#issues) webhook.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> Result<crate::types::Issue> {
        let url = format!(
            "/repos/{}/{}/issues/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Update an issue.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/issues/{issue_number}` endpoint.
     *
     * Issue owners and users with push access can edit an issue.
     *
     * FROM: <https://docs.github.com/rest/reference/issues/#update-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn update(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesUpdateRequest,
    ) -> Result<crate::types::Issue> {
        let url = format!(
            "/repos/{}/{}/issues/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Add assignees to an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/assignees` endpoint.
     *
     * Adds up to 10 assignees to an issue. Users already assigned to an issue are not replaced.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#add-assignees-to-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn add_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddAssigneesRequest,
    ) -> Result<crate::types::IssueSimple> {
        let url = format!(
            "/repos/{}/{}/issues/{}/assignees",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Remove assignees from an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/assignees` endpoint.
     *
     * Removes one or more assignees from an issue.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#remove-assignees-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn remove_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesRemoveAssigneesRequest,
    ) -> Result<crate::types::IssueSimple> {
        let url = format!(
            "/repos/{}/{}/issues/{}/assignees",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List issue comments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/comments` endpoint.
     *
     * Issue Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-comments>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::IssueComment>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/comments?page={}&per_page={}&since={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
            format!("{}", page),
            format!("{}", per_page),
            since,
        );

        self.client.get(&url).await
    }

    /**
     * Create an issue comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/comments` endpoint.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in abuse rate limiting. See "[Abuse rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#abuse-rate-limits)" and "[Dealing with abuse rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-abuse-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesUpdateCommentRequest,
    ) -> Result<crate::types::IssueComment> {
        let url = format!(
            "/repos/{}/{}/issues/{}/comments",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List issue events.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-events>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_events(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::IssueEventFor>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/events?page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }

    /**
     * List labels for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_labels_on_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels?page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }

    /**
     * Set labels for an issue.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     * Removes any previous labels and sets the new labels for an issue.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#set-labels-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn set_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddLabelsRequest,
    ) -> Result<Vec<crate::types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Add labels to an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#add-labels-to-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn add_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddLabelsRequest,
    ) -> Result<Vec<crate::types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Remove all labels from an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#remove-all-labels-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn remove_all_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Remove a label from an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels/{name}` endpoint.
     *
     * Removes the specified label from the issue, and returns the remaining labels on the issue. This endpoint returns a `404 Not Found` status if the label does not exist.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#remove-a-label-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `name: &str`
     */
    pub async fn remove_label(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        name: &str,
    ) -> Result<Vec<crate::types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
            crate::progenitor_support::encode_path(&name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Lock an issue.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/issues/{issue_number}/lock` endpoint.
     *
     * Users with push access can lock an issue or pull request's conversation.
     *
     * Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/issues#lock-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn lock(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesLockRequest,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/{}/lock",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Unlock an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/lock` endpoint.
     *
     * Users with push access can unlock an issue's conversation.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#unlock-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn unlock(&self, owner: &str, repo: &str, issue_number: i64) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/{}/lock",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List timeline events for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/timeline` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-timeline-events-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_events_for_timeline(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::TimelineIssueEvents>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/timeline?page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&issue_number.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }

    /**
     * List labels for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/labels` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_labels_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Label>> {
        let url = format!(
            "/repos/{}/{}/labels?page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }

    /**
     * Create a label.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/labels` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_label(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::IssuesCreateLabelRequest,
    ) -> Result<crate::types::Label> {
        let url = format!(
            "/repos/{}/{}/labels",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get a label.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/labels/{name}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `name: &str`
     */
    pub async fn get_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<crate::types::Label> {
        let url = format!(
            "/repos/{}/{}/labels/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&name.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Delete a label.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/labels/{name}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#delete-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `name: &str`
     */
    pub async fn delete_label(&self, owner: &str, repo: &str, name: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/labels/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&name.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a label.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/labels/{name}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#update-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `name: &str`
     */
    pub async fn update_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        body: &crate::types::IssuesUpdateLabelRequest,
    ) -> Result<crate::types::Label> {
        let url = format!(
            "/repos/{}/{}/labels/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&name.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List milestones.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-milestones>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `state: crate::types::IssuesListState` -- The state of the milestone. Either `open`, `closed`, or `all`.
     * * `sort: crate::types::IssuesListMilestonesSort` -- What to sort results by. Either `due_on` or `completeness`.
     * * `direction: crate::types::Direction` -- The direction of the sort. Either `asc` or `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_milestones(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::IssuesListState,
        sort: crate::types::IssuesListMilestonesSort,
        direction: crate::types::Direction,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Milestone>> {
        let url = format!(
            "/repos/{}/{}/milestones?direction={}&page={}&per_page={}&sort={}&state={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            direction,
            format!("{}", page),
            format!("{}", per_page),
            sort,
            state,
        );

        self.client.get(&url).await
    }

    /**
     * Create a milestone.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/milestones` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_milestone(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::IssuesCreateMilestoneRequest,
    ) -> Result<crate::types::Milestone> {
        let url = format!(
            "/repos/{}/{}/milestones",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get a milestone.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones/{milestone_number}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
     */
    pub async fn get_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
    ) -> Result<crate::types::Milestone> {
        let url = format!(
            "/repos/{}/{}/milestones/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&milestone_number.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Delete a milestone.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/milestones/{milestone_number}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#delete-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
     */
    pub async fn delete_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/milestones/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&milestone_number.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a milestone.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/milestones/{milestone_number}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#update-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
     */
    pub async fn update_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
        body: &crate::types::IssuesUpdateMilestoneRequest,
    ) -> Result<crate::types::Milestone> {
        let url = format!(
            "/repos/{}/{}/milestones/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&milestone_number.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List labels for issues in a milestone.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones/{milestone_number}/labels` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-issues-in-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_labels_for_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Label>> {
        let url = format!(
            "/repos/{}/{}/milestones/{}/labels?page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&milestone_number.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }

    /**
     * List user account issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/user/issues` endpoint.
     *
     * List issues across owned and member repositories assigned to the authenticated user.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-user-account-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. Can be one of:  
     *  \* `assigned`: Issues assigned to you  
     *  \* `created`: Issues created by you  
     *  \* `mentioned`: Issues mentioning you  
     *  \* `subscribed`: Issues you're subscribed to updates for  
     *  \* `all`: All issues the authenticated user can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Direction` -- One of `asc` (ascending) or `desc` (descending).
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_authenticated_user(
        &self,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Direction,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Issue>> {
        let url = format!(
            "/user/issues?direction={}&filter={}&labels={}&page={}&per_page={}&since={}&sort={}&\
             state={}",
            direction,
            filter,
            labels.to_string(),
            format!("{}", page),
            format!("{}", per_page),
            since,
            sort,
            state,
        );

        self.client.get(&url).await
    }
}