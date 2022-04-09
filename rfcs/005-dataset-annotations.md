# RFC-005: New Annotation Metadata Events

**Start Date**: 2022-04-08

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/8?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/20)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/19?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/21)

## Summary

This RFC proposes to extend the set of metadata events with new types used for annotating datasets, such as adding description, licenses, and attachments.

## Motivation

Existing metadata events focus primarily on data pipeline functionality like ingestion and transformation of data, but we need to start expanding into governance/stewardship aspects. This first step will add event types that will let dataset authors provide basic human-oriented information about datasets.

## Guide-level explanation

Example of annotating a dataset:

```yaml
kind: DatasetSnapshot
version: 1
content:
  name: ca.ontario.covid19.case-details
  kind: root
  metadata:
    - kind: setPollingSource
      ...
    - kind: setInfo
      description: Confirmed positive cases of COVID-19 in Ontario
      keywords:
        - Healthcare
        - Epidemiology
        - COVID-19
        - SARS-CoV-2
        - Disaggregated
        - Anonymized
        - Ontario
        - Canada
    - kind: setLicense
      name: Open Government Licence – Ontario
      url: https://www.ontario.ca/page/open-government-licence-ontario
    - kind: setAttachments
      attachments:
        kind: embedded
        items:
          - path: README.md
            body: |
              # Confirmed positive cases of COVID-19 in Ontario
              Detailed description follows...
```

## Reference-level explanation

Following extension events will be added to the specification:

| Event Type       | Description                                                                                                                |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `SetInfo`        | (Optional extension, unstable) Provides basic human-readable description of a dataset                                      |
| `SetLicense`     | (Optional extension, unstable) Defines the dataset license.                                                                |
| `SetAttachments` | (Optional extension, unstable) Associates a set of files with this dataset (readme, notebooks, additional metadata, etc.). |

## Drawbacks

- More event types to deal with


## Rationale and alternatives

We are entering the territory of metadata where a lot of existing standards exist: `schema.org`, Dublin Core, and many others dealing with dataset descriptions. 

This presents a few options:
- Define our own schemas 
  - Easy for development, but will result in poor interoperability
- Define our own schemas, but align them with existing standards
  - Better interop, but puts burden on chosing the right standards to follow and keeping up with their evolution
- Start with an existing standard
  - It's likely that some aspects we need will not be covered and we'll have to extend and customize
  - We may not be able to express some standard schemas in our strongly-typed data model

We decide to delay the decision for now until we do more research on standards and investigate replacing data model with `IPLD`.

In addition to basic annotational metadata we will also have files associated with datasets. These could be:
- A simple README file in different text formats
- Images used in the README
- A set of notebooks that demonstrate how to use datasets

We do not want to re-invent version control systems in ODF metadata chain - therefore we will associate files with datasets by referencing external systems such as `git` repositories. For simple cases we will allow embedding files directly into metadata (e.g. ability to provide README as in example above).

## Prior art

## Unresolved questions

- **Balance between schema and flexibility** - it should be easy for users to extend the metadata for their own governance needs. We should either allows some free-form component in the existing metadata events, or create separate event types.

## Future possibilities

- Information from new events can be indexed and used for full-text search
- License change events should be one of those that trigger notifications for downstream consumers
- All textual fields should be extended with internationalization support
